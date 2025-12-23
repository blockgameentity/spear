use crate::constants::*;
use crate::gui::overlay_ui::*;

use egui_overlay;
use egui_overlay::EguiOverlay;
use egui_overlay::egui_render_three_d::ThreeDBackend;
use egui_overlay::egui_window_glfw_passthrough::{GlfwBackend, GlfwConfig};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tokio;

struct Overlay;

impl EguiOverlay for Overlay {
    fn gui_run(
        &mut self,
        egui_context: &egui::Context,
        _default_gfx_backend: &mut ThreeDBackend,
        glfw_backend: &mut GlfwBackend,
    ) {
        let overlay_hwnd = if let Ok(window_handle) = glfw_backend.window.window_handle() {
            if let RawWindowHandle::Win32(handle) = window_handle.as_raw() {
                handle.hwnd.get() as winapi::shared::windef::HWND
            } else {
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        };
        crate::gui::overlay::update(egui_context, overlay_hwnd);
        let input_needed = *crate::gui::overlay::INPUT_NEEDED.lock().unwrap();
        if input_needed {
            glfw_backend.set_passthrough(false);
        } else {
            glfw_backend.set_passthrough(true);
        }
        let ui_ready = *UI_READY.lock().unwrap();
        unsafe {
            winapi::um::winuser::ShowWindow(
                overlay_hwnd,
                if ui_ready {
                    winapi::um::winuser::SW_SHOW
                } else {
                    winapi::um::winuser::SW_HIDE
                },
            );
        }
    }
}

pub unsafe fn overlay_thread_func(
    _lp_param: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    log::info!("[+] Overlay thread started");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let path = SPEAR_PATH.join("peacock").join("chunk0.js");
    if !path.exists() {
        std::fs::create_dir_all(SPEAR_PATH.join("peacock")).unwrap();
        let version = rt
            .block_on(crate::core::resources::peacock_download_release())
            .unwrap_or_else(|e| {
                log::error!("Failed to download peacock: {}", e);
                "unknown".to_string()
            });
        *crate::gui::overlay_ui::PEACOCK_VERSION.lock().unwrap() =
            Some(version.trim_start_matches('v').to_string());
    } else {
        let version_path = SPEAR_PATH.join("peacock").join("version.txt");
        if let Ok(version) = std::fs::read_to_string(version_path) {
            *crate::gui::overlay_ui::PEACOCK_VERSION.lock().unwrap() =
                Some(version.trim().trim_start_matches('v').to_string());
        } else {
            *crate::gui::overlay_ui::PEACOCK_VERSION.lock().unwrap() = Some("unknown".to_string());
        }
    }

    log::info!("[+] Setting up overlay...");

    let cache_dir = SPEAR_PATH.join("cache");
    log::info!("Cache dir: {:?}", cache_dir);
    log::info!("[+] Creating cache directory...");
    std::fs::create_dir_all(&cache_dir).ok();
    log::info!("[+] Cache directory ready");

    let icon_path = cache_dir.join("play_icon.png");
    let settings_icon_path = cache_dir.join("settings_icon.png");
    let font_path = cache_dir.join("font.ttf");

    log::info!("[+] Checking for cached resources...");
    if icon_path.exists() {
        log::info!("[+] Loading icon from cache...");
        let data = std::fs::read(icon_path).unwrap();
        *crate::core::resources::PLAY_ICON_DATA.lock().unwrap() = Some(data);
        log::info!("[+] Loaded icon from cache");
    } else {
        log::info!("[+] Icon not cached");
    }
    if settings_icon_path.exists() {
        log::info!("[+] Loading settings icon from cache...");
        let data = std::fs::read(settings_icon_path).unwrap();
        *crate::core::resources::SETTINGS_ICON_DATA.lock().unwrap() = Some(data);
        log::info!("[+] Loaded settings icon from cache");
    } else {
        log::info!("[+] Settings icon not cached");
    }
    if font_path.exists() {
        log::info!("[+] Loading font from cache...");
        let data = std::fs::read(font_path).unwrap();
        *crate::core::resources::FONT_DATA.lock().unwrap() = Some(data);
        log::info!("[+] Loaded font from cache");
    } else {
        log::info!("[+] Font not cached");
    }

    if crate::core::resources::PLAY_ICON_DATA
        .lock()
        .unwrap()
        .is_none()
        || crate::core::resources::SETTINGS_ICON_DATA
            .lock()
            .unwrap()
            .is_none()
        || crate::core::resources::FONT_DATA.lock().unwrap().is_none()
    {
        let exe_path = std::env::current_exe().unwrap();
        crate::pe::parsing::parse_pe_resources(&exe_path);
    } else {
        log::info!("[+] All resources loaded from cache");
    }

    let main_hwnd = unsafe { crate::gui::find_main_window() };
    if let Some(main_hwnd) = main_hwnd {
        let mut main_rect = unsafe { std::mem::zeroed() };
        unsafe { winapi::um::winuser::GetWindowRect(main_hwnd, &mut main_rect) };
        let width = (main_rect.right - main_rect.left) as f32;
        let height = (main_rect.bottom - main_rect.top) as f32;
        *WIDTH.lock().unwrap() = width;
        *HEIGHT.lock().unwrap() = height;
        *MAIN_HWND.lock().unwrap() = Some(main_hwnd as usize);
        log::info!(
            "[+] Main window position: ({}, {}), size: {}x{}",
            main_rect.left,
            main_rect.top,
            width,
            height
        );
    }

    log::info!("[+] Starting egui_overlay...");
    let config = GlfwConfig {
        glfw_callback: Box::new(|gtx| {
            (egui_overlay::egui_window_glfw_passthrough::GlfwConfig::default().glfw_callback)(gtx);
            gtx.window_hint(
                egui_overlay::egui_window_glfw_passthrough::glfw::WindowHint::ScaleToMonitor(false),
            );
        }),
        transparent_window: Some(true),
        disable_content_scaling: true,
        ..Default::default()
    };
    egui_overlay::start(Overlay, config);
    log::info!("[+] Eframe finished");
    0
}
