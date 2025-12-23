
use crate::gui::overlay_ui::*;
use crate::gui::settings::show_settings_window;

use egui;
use lazy_static::lazy_static;
use std::sync::Mutex;
use winapi::um::dwmapi::DwmSetWindowAttribute;

lazy_static! {
    pub static ref INPUT_NEEDED: Mutex<bool> = Mutex::new(false);
}

pub fn update(egui_ctx: &egui::Context, overlay_hwnd: winapi::shared::windef::HWND) {
    {
        let main_hwnd = (*MAIN_HWND.lock().unwrap()).map(|h| h as winapi::shared::windef::HWND);
        let mut set_parent_done = SET_PARENT_DONE.lock().unwrap();
        if !*set_parent_done {
            if let Some(main_hwnd) = main_hwnd {
                if !overlay_hwnd.is_null() {
                    unsafe { winapi::um::winuser::SetParent(overlay_hwnd, main_hwnd) };
                    let style = unsafe {
                        winapi::um::winuser::GetWindowLongPtrA(
                            overlay_hwnd,
                            winapi::um::winuser::GWL_STYLE,
                        )
                    } as u32;
                    unsafe {
                        winapi::um::winuser::SetWindowLongPtrA(
                            overlay_hwnd,
                            winapi::um::winuser::GWL_STYLE,
                            (style | winapi::um::winuser::WS_CHILD) as _,
                        )
                    };
                    unsafe {
                        let mut preference = crate::constants::DWMWCP_DONOTROUND as u32;
                        DwmSetWindowAttribute(
                            overlay_hwnd,
                            crate::constants::DWMWA_WINDOW_CORNER_PREFERENCE,
                            &mut preference as *mut _ as *mut _,
                            std::mem::size_of::<u32>() as u32,
                        );
                    }
                    let hrgn = unsafe {
                        winapi::um::wingdi::CreateRectRgn(
                            0,
                            0,
                            *WIDTH.lock().unwrap() as i32,
                            *HEIGHT.lock().unwrap() as i32,
                        )
                    };
                    unsafe { winapi::um::winuser::SetWindowRgn(overlay_hwnd, hrgn, 1) };
                    let ex_style = unsafe {
                        winapi::um::winuser::GetWindowLongPtrA(
                            overlay_hwnd,
                            winapi::um::winuser::GWL_EXSTYLE,
                        )
                    } as u32;
                    unsafe {
                        winapi::um::winuser::SetWindowLongPtrA(
                            overlay_hwnd,
                            winapi::um::winuser::GWL_EXSTYLE,
                            (ex_style | winapi::um::winuser::WS_EX_LAYERED) as _,
                        )
                    };
                    unsafe {
                        winapi::um::winuser::SetLayeredWindowAttributes(
                            overlay_hwnd,
                            0,
                            255,
                            winapi::um::winuser::LWA_ALPHA,
                        )
                    };
                    unsafe {
                        winapi::um::winuser::SetWindowPos(
                            overlay_hwnd,
                            std::ptr::null_mut(),
                            0,
                            0,
                            0,
                            0,
                            winapi::um::winuser::SWP_NOSIZE | winapi::um::winuser::SWP_NOZORDER,
                        );
                    }
                    *set_parent_done = true;
                }
            }
        }
    }

    render_overlay_ui(egui_ctx);

    if *crate::gui::overlay_ui::SHOULD_SPAWN_EXTRA.lock().unwrap()
        && *crate::gui::overlay_ui::UI_READY.lock().unwrap()
        && !*crate::gui::overlay_ui::EXTRA_SPAWNED.lock().unwrap()
    {
        std::thread::spawn(|| {
            show_settings_window();
        });
        *crate::gui::overlay_ui::EXTRA_SPAWNED.lock().unwrap() = true;
        *crate::gui::overlay_ui::SHOULD_SPAWN_EXTRA.lock().unwrap() = false;
    }
}
