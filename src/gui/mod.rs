#[allow(dead_code)]
pub mod debug;
#[allow(dead_code)]
pub mod gdi;
#[allow(dead_code)]
pub mod overlay;
#[allow(dead_code)]
pub mod overlay_thread;
pub mod overlay_ui;
pub mod overlay_utils;
pub mod settings;

use lazy_static::lazy_static;
use std::sync::Mutex;
use winapi::um::winuser::GetClassNameW;

lazy_static! {
    pub static ref SHOW_EXTRA_WINDOW: Mutex<bool> = Mutex::new(false);
}

pub unsafe fn find_main_window() -> Option<winapi::shared::windef::HWND> {
    use winapi::shared::windef::HWND;
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowThreadProcessId};

    let mut main_hwnd = None;
    let _current_pid = unsafe { GetCurrentProcessId() };

    unsafe extern "system" fn enum_proc(
        hwnd: HWND,
        lparam: winapi::shared::minwindef::LPARAM,
    ) -> winapi::shared::minwindef::BOOL {
        let main_hwnd_ptr = lparam as *mut Option<HWND>;
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
        if pid == unsafe { GetCurrentProcessId() } {
            let mut class = [0u16; 256];
            unsafe { GetClassNameW(hwnd, class.as_mut_ptr(), class.len() as i32) };
            let class_str = String::from_utf16_lossy(&class);
            let mut title = [0u16; 256];
            unsafe { GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32) };
            let title_str = String::from_utf16_lossy(&title);
            log::info!(
                "[+] Found window: class='{}', title='{}', pid={}",
                class_str,
                title_str,
                pid
            );
            if !title_str.is_empty()
                && !title_str.contains("Overlay")
                && (title_str.contains("HITMAN") || class_str == "Launcher")
                && !class_str.contains("ConsoleWindowClass")
                && !class_str.contains("Winit Thread Event Target")
            {
                unsafe { *main_hwnd_ptr = Some(hwnd) };
                log::info!(
                    "[+] Selected main window: class='{}', title='{}', {:?}",
                    class_str,
                    title_str,
                    hwnd
                );
                return winapi::shared::minwindef::FALSE;
            }
        }
        winapi::shared::minwindef::TRUE
    }

    unsafe {
        EnumWindows(
            Some(enum_proc),
            &mut main_hwnd as *mut _ as winapi::shared::minwindef::LPARAM,
        );
    }
    main_hwnd
}
