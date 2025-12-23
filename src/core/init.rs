use backtrace::Backtrace;
use chrono;
use log;
use simplelog::*;
use std::ffi::CString;
use std::fs;
use std::mem::{size_of, zeroed};
use std::os::windows::process::CommandExt;
use std::panic::catch_unwind;
use std::process::Stdio;

use std::thread;
use std::time::Duration;
use sysinfo::System;
use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::jobapi2::{AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject};
use winapi::um::processthreadsapi::{
    CreateProcessA, OpenProcess, PROCESS_INFORMATION, STARTUPINFOA,
};
use winapi::um::winbase::{STARTF_USESHOWWINDOW, STARTF_USESTDHANDLES};
use winapi::um::winnt::{
    HANDLE, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JobObjectExtendedLimitInformation, PROCESS_ALL_ACCESS,
};
use winapi::um::winuser::{SW_HIDE, SW_SHOW, SetForegroundWindow, ShowWindow};

#[cfg(debug_assertions)]
pub fn setup_logging() {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let log_filename = format!("spear_log_{}.txt", timestamp);
    let file_logger: Box<dyn SharedLogger> = WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        std::fs::File::create(&log_filename).unwrap(),
    );
    let mut loggers = vec![file_logger];
    let term_logger: Box<dyn SharedLogger> = TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    loggers.push(term_logger);
    CombinedLogger::init(loggers).unwrap();

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();
        log::error!("Panic occurred: {:?}", panic_info);
        log::error!("Backtrace:\n{:?}", bt);
    }));
}

pub fn initialize_everything() {
    unsafe {
        log::info!("[+] DLL injected successfully - init thread running");

        log::info!("[+] Initializing hooks");
        log::info!("[+] Creating hooks...");

        crate::REAL_WINMM = winapi::um::libloaderapi::LoadLibraryA(
            b"c:\\windows\\system32\\winmm.dll\0".as_ptr() as *const i8,
        );
        log::info!(
            "[+] Loaded real winmm.dll @ 0x{:x}",
            crate::REAL_WINMM as usize
        );

        let _ = catch_unwind(|| crate::hooks::setup_hooks());

        crate::core::resources::allocate_fake_resource();

        loop {
            if let Some(main_hwnd) = crate::gui::find_main_window() {
                log::info!("[+] Main window found, hiding it until UI ready...");
                ShowWindow(main_hwnd, SW_HIDE);
                let main_hwnd_usize = main_hwnd as usize;
                std::thread::spawn(move || {
                    let main_hwnd = main_hwnd_usize as winapi::shared::windef::HWND;
                    loop {
                        if *crate::gui::overlay_ui::UI_READY.lock().unwrap() {
                            ShowWindow(main_hwnd, SW_SHOW);
                            SetForegroundWindow(main_hwnd);
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                });
                log::info!("[+] Overlay thread spawned");
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        log::info!("running overlay thread func");

        crate::gui::overlay_thread::overlay_thread_func(std::ptr::null_mut());
    }
    log::info!("[+] Initialization complete");
}

pub fn spawn_watchdog() {
    log::info!("[+] Spawning watchdog");
    let spear_path = crate::constants::SPEAR_PATH.clone();
    if !spear_path.exists() {
        fs::create_dir_all(&spear_path).unwrap();
    }

    log::info!("[+] Creating job object");
    let job_handle = unsafe { CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()) };
    if job_handle.is_null() {
        log::error!("[!] Failed to create job object");
        return;
    }
    log::info!("[+] Job object created successfully");

    log::info!("[+] Setting job object to kill on close");
    let mut limit_info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
    limit_info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    unsafe {
        SetInformationJobObject(
            job_handle,
            JobObjectExtendedLimitInformation,
            &mut limit_info as *mut _ as *mut winapi::ctypes::c_void,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );
    }
    log::info!("[+] Job object configured to kill on close");

    // Start node server
    log::info!("[+] Starting node server");
    let node_path = spear_path.join("peacock").join("nodedist").join("node.exe");
    let server = std::process::Command::new(&node_path)
        .arg("chunk0.js")
        .current_dir(&spear_path.join("peacock"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
        .expect("Failed to start server");
    log::info!("[+] Node server started with PID {}", server.id());

    let server_pid = server.id();
    let server_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, server_pid) };
    if server_handle != 0 as HANDLE {
        let assign_result = unsafe { AssignProcessToJobObject(job_handle, server_handle) };
        if assign_result != 0 {
            log::info!("[+] Server assigned to job object");
        } else {
            log::error!("[!] Failed to assign server to job object");
        }
        unsafe { CloseHandle(server_handle) };
    } else {
        log::error!("[!] Failed to open server process handle");
    }

    log::info!("[+] Starting PeacockPatcher");
    let patcher_path = spear_path.join("peacock").join("PeacockPatcher.exe");
    let mut si: STARTUPINFOA = unsafe { zeroed() };
    si.cb = size_of::<STARTUPINFOA>() as u32;
    si.dwFlags = STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES;
    si.wShowWindow = SW_HIDE as u16;
    si.hStdInput = INVALID_HANDLE_VALUE;
    si.hStdOutput = INVALID_HANDLE_VALUE;
    si.hStdError = INVALID_HANDLE_VALUE;
    let mut pi: PROCESS_INFORMATION = unsafe { zeroed() };
    let application_name = std::ptr::null::<i8>();
    let command_line = CString::new(format!("\"{}\"", patcher_path.display())).unwrap();
    unsafe {
        CreateProcessA(
            application_name,
            command_line.as_ptr() as *mut i8,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            FALSE,
            0,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut si,
            &mut pi,
        );
        CloseHandle(pi.hThread);
        let assign_result = AssignProcessToJobObject(job_handle, pi.hProcess);
        if assign_result != 0 {
            log::info!("[+] PeacockPatcher assigned to job object");
        } else {
            log::error!("[!] Failed to assign PeacockPatcher to job object");
        }
        CloseHandle(pi.hProcess);
    }
    log::info!("[+] PeacockPatcher started");

    log::info!("[+] Waiting to assign HITMAN3.exe to job object");
    let mut hitman_assigned = false;
    while !hitman_assigned {
        let mut system = System::new_all();
        system.refresh_all();
        if let Some(hitman) = system
            .processes()
            .values()
            .find(|p| p.name() == "HITMAN3.exe")
        {
            let pid = unsafe { *(&hitman.pid() as *const sysinfo::Pid as *const u32) };
            log::info!("[+] Found HITMAN3.exe with PID {}", pid);
            let hitman_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };
            if hitman_handle != 0 as HANDLE {
                let assign_result = unsafe { AssignProcessToJobObject(job_handle, hitman_handle) };
                if assign_result != 0 {
                    hitman_assigned = true;
                    log::info!("[+] HITMAN3.exe assigned to job object");
                } else {
                    log::error!("[!] Failed to assign HITMAN3.exe to job object");
                }
                unsafe { CloseHandle(hitman_handle) };
            } else {
                log::error!("[!] Failed to open HITMAN3.exe process handle");
            }
        }
        if !hitman_assigned {
            thread::sleep(Duration::from_millis(100));
        }
    }

    log::info!("[+] Watchdog setup complete - job object will manage process lifecycle");
}
