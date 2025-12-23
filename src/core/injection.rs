use aobscan::Pattern;
use dll_syringe::{Syringe, process::OwnedProcess};
use num_cpus;
use std::ptr;
use std::slice;
use sysinfo::System;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winnt::PAGE_READWRITE;

use crate::gui::overlay_utils::get_text_section;

// AOB Patterns
//
// Function for PLAY button
const PATTERN_FUN_140014684: &[u8] = &[
    0x40, 0x53, 0x48, 0x81, 0xEC, 0xA0, 0x01, 0x00, 0x00, 0x48, 0x8B, 0x05, 0xE4, 0xA9, 0x03, 0x00,
    0x48, 0x33, 0xC4, 0x48, 0x89, 0x84, 0x24, 0x90, 0x01, 0x00, 0x00, 0x48, 0x8B, 0xD9, 0xBA, 0x02,
    0x7F, 0x00, 0x00, 0x33, 0xC9, 0xFF, 0x15, 0xF1, 0x5E, 0x02, 0x00,
];

// The part that closes the window in the play button function
const PATTERN_CALL: &[u8] = &[
    0x84, 0xC0, 0x74, 0x08, 0x48, 0x8B, 0xCB, 0xE8, 0xF6, 0x49, 0xFF, 0xFF,
];

const REPLACEMENT_CALL: &[u8] = &[
    0x84, 0xC0, 0x74, 0x08, 0x48, 0x8B, 0xCB, 0x90, 0x90, 0x90, 0x90, 0x90,
];

pub fn perform_injection() {
    log::info!("[+] PLAY button clicked - disabling button and scanning for patterns");

    if let Some((text_base, text_size)) = get_text_section() {
        let text_data = unsafe { slice::from_raw_parts(text_base, text_size) };
        log::info!(
            "[+] Scanning .text section (base: {:p}, size: {})",
            text_base,
            text_size
        );

        let pattern1 = Pattern::new(
            PATTERN_FUN_140014684.to_vec(),
            vec![true; PATTERN_FUN_140014684.len()],
            num_cpus::get(),
        );
        let mut found1 = None;
        pattern1.scan(text_data, |offset| {
            found1 = Some(offset);
            false
        });
        if let Some(offset) = found1 {
            let addr = unsafe { text_base.add(offset) };
            log::info!(
                "[+] Found FUN_140014684 at offset {} (address: {:p})",
                offset,
                addr
            );
        } else {
            log::error!("[!] Pattern for FUN_140014684 not found");
        }

        let pattern2 = Pattern::new(
            PATTERN_CALL.to_vec(),
            vec![true; PATTERN_CALL.len()],
            num_cpus::get(),
        );
        let mut found2 = None;
        pattern2.scan(text_data, |offset| {
            found2 = Some(offset);
            false
        });
        if let Some(offset) = found2 {
            let addr = unsafe { text_base.add(offset) };
            log::info!(
                "[+] Found CALL FUN_1400090ec at offset {} (address: {:p}) - applying NOP patch",
                offset,
                addr
            );

            let mut old_protect: u32 = 0;
            if unsafe {
                VirtualProtect(
                    addr as *mut _,
                    REPLACEMENT_CALL.len(),
                    PAGE_READWRITE,
                    &mut old_protect,
                )
            } != 0
            {
                unsafe {
                    ptr::copy_nonoverlapping(
                        REPLACEMENT_CALL.as_ptr(),
                        addr as *mut u8,
                        REPLACEMENT_CALL.len(),
                    )
                };
                log::info!("[+] NOP patch applied successfully");

                unsafe {
                    VirtualProtect(
                        addr as *mut _,
                        REPLACEMENT_CALL.len(),
                        old_protect,
                        &mut old_protect,
                    )
                };

                if let Some(offset) = found1 {
                    let func_addr = unsafe { text_base.add(offset) };
                    let func: extern "C" fn() = unsafe { std::mem::transmute(func_addr) };
                    log::info!("[+] Calling FUN_140014684 at {:p}", func_addr);
                    func();

                    log::info!("[+] Waiting for HITMAN3.exe to start...");
                    let mut system = System::new_all();
                    loop {
                        system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                        if system
                            .processes()
                            .values()
                            .any(|p| p.name() == "HITMAN3.exe")
                        {
                            log::info!("[+] HITMAN3.exe found in process list");
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }

                    log::info!("[+] Injecting winmm.dll into HITMAN3.exe");
                    if let Some(owned_process) = OwnedProcess::find_first_by_name("HITMAN3.exe") {
                        match Syringe::for_process(owned_process).inject("winmm.dll") {
                            Ok(_) => {
                                log::info!("[+] Successfully injected winmm.dll");
                                unsafe {
                                    ExitProcess(0);
                                }
                            }
                            Err(e) => log::error!("[!] Failed to inject winmm.dll: {:?}", e),
                        }
                    } else {
                        log::error!("[!] Failed to find HITMAN3.exe process for injection");
                    }
                }
            } else {
                log::error!("[!] Failed to change memory protection for patching");
            }
        } else {
            log::error!("[!] Pattern for CALL FUN_1400090ec not found");
        }
    } else {
        log::error!("[!] Failed to get .text section");
    }
}
