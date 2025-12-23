use std::sync::Once;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, GetModuleFileNameW};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod config;
mod constants;
mod core;
mod exports;
mod gui;
mod hooks;
mod pe;

pub static mut REAL_WINMM: HINSTANCE = 0 as HINSTANCE;

static INIT: Once = Once::new();

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "system" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(hinst);
        #[cfg(debug_assertions)]
        winapi::um::consoleapi::AllocConsole();

        let _ = std::thread::spawn(|| {
            INIT.call_once(|| {
                #[cfg(debug_assertions)]
                crate::core::init::setup_logging();

                let mut exe_path = [0u16; 260];
                let len = GetModuleFileNameW(
                    std::ptr::null_mut(),
                    exe_path.as_mut_ptr(),
                    exe_path.len() as u32,
                );
                let exe_name = if len > 0 {
                    let exe_str = String::from_utf16_lossy(&exe_path[..len as usize]);
                    let path = std::path::Path::new(&exe_str);
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                } else {
                    "".to_string()
                };

                log::info!("[+] Detected exe_name: {}", exe_name);

                if exe_name == "Launcher.exe" {
                    crate::core::init::initialize_everything();
                } else if exe_name == "hitman3.exe" {
                    crate::core::init::spawn_watchdog();
                }
            });
        });
    }
    TRUE
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, process::Command};

    #[test]
    fn test_build_and_deploy() {
        let build_status = Command::new("cargo")
            .args(&["build", "-p", "spearlauncher"])
            .current_dir("../../")
            .status()
            .expect("Failed to run cargo build");

        if !build_status.success() {
            eprintln!("Cargo build failed");
            return;
        }

        let tasklist_output = Command::new("tasklist")
            .args(&["/fi", "imagename eq Launcher.exe", "/nh"])
            .output()
            .expect("Failed to run tasklist");

        let output_str = String::from_utf8_lossy(&tasklist_output.stdout);

        if output_str.contains("Launcher.exe") {
            println!("Launcher.exe is running, killing it...");
            Command::new("taskkill")
                .args(&["/f", "/im", "Launcher.exe"])
                .status()
                .expect("Failed to kill Launcher.exe");
            println!("Launcher.exe killed");
        }

        let src_path = Path::new("../../target").join("debug").join("spear.dll");
        let dst_dir = Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\HITMAN 3");
        fs::create_dir_all(&dst_dir).expect("Failed to create destination directory");
        let dst_path = dst_dir.join("winmm.dll");

        if !src_path.exists() {
            eprintln!("Source DLL not found at {:?}", src_path);
            return;
        }

        fs::copy(&src_path, &dst_path).expect("Failed to copy DLL");
        println!("winmm.dll copied to HITMAN 3 directory");

        println!("Launching HITMAN 3 via Steam...");
        Command::new("cmd")
            .args(&["/c", "start", "steam://run/1659040"])
            .status()
            .expect("Failed to launch HITMAN 3 via Steam");
        println!("HITMAN 3 launched");
    }
}
