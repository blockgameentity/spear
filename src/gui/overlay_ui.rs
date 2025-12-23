use crate::core::injection::perform_injection;
use crate::core::resources::*;
use crate::gui::debug::DEBUG_PARAMS;
use crate::gui::gdi::text_rendering;

use egui;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref ICON_TEXTURE: Mutex<Option<egui::TextureHandle>> = Mutex::new(None);
    pub static ref SETTINGS_ICON_TEXTURE: Mutex<Option<egui::TextureHandle>> = Mutex::new(None);
    pub static ref TEXT_TEXTURE: Mutex<Option<egui::TextureHandle>> = Mutex::new(None);
    pub static ref TEXT_SIZE: Mutex<egui::Vec2> = Mutex::new(egui::Vec2::ZERO);
    pub static ref MAIN_HWND: Mutex<Option<usize>> = Mutex::new(None);
    pub static ref SET_PARENT_DONE: Mutex<bool> = Mutex::new(false);
    pub static ref WIDTH: Mutex<f32> = Mutex::new(1920.0);
    pub static ref HEIGHT: Mutex<f32> = Mutex::new(1080.0);
    pub static ref UI_READY: Mutex<bool> = Mutex::new(false);
    pub static ref SHOULD_SPAWN_EXTRA: Mutex<bool> = Mutex::new(false);
    pub static ref EXTRA_SPAWNED: Mutex<bool> = Mutex::new(false);
    pub static ref PLAY_BUTTON_DISABLED: Mutex<bool> = Mutex::new(false);
    pub static ref PEACOCK_VERSION_TEXTURE: Mutex<Option<egui::TextureHandle>> = Mutex::new(None);
    pub static ref PEACOCK_VERSION_SIZE: Mutex<egui::Vec2> = Mutex::new(egui::Vec2::ZERO);
    pub static ref PEACOCK_VERSION: Mutex<Option<String>> = Mutex::new(None);
}

pub fn render_overlay_ui(egui_ctx: &egui::Context) {
    let mut visuals = egui::Visuals::default();
    visuals.clip_rect_margin = 0.0;
    visuals.popup_shadow = egui::Shadow::NONE;
    egui_ctx.set_visuals(visuals);
    egui_ctx.tessellation_options_mut(|opts| {
        opts.feathering = false;
    });

    {
        let mut icon_texture = ICON_TEXTURE.lock().unwrap();
        if icon_texture.is_none() {
            if let Some(data) = &*PLAY_ICON_DATA.lock().unwrap() {
                log::info!("[+] Loading play icon texture...");
                let img = image::load_from_memory(data).unwrap().to_rgba8();
                let size = [img.width() as usize, img.height() as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &img.into_raw());
                *icon_texture = Some(egui_ctx.load_texture(
                    "play_icon",
                    color_image,
                    egui::TextureOptions::default(),
                ));
                log::info!("[+] Play icon texture loaded");
            } else {
                log::info!("[!] Play icon data not available");
            }
        }
    }

    {
        let mut settings_icon_texture = SETTINGS_ICON_TEXTURE.lock().unwrap();
        if settings_icon_texture.is_none() {
            if let Some(data) = &*SETTINGS_ICON_DATA.lock().unwrap() {
                log::info!("[+] Loading settings icon texture...");
                let img = image::load_from_memory(data).unwrap().to_rgba8();
                let size = [img.width() as usize, img.height() as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &img.into_raw());
                *settings_icon_texture = Some(egui_ctx.load_texture(
                    "settings_icon",
                    color_image,
                    egui::TextureOptions::default(),
                ));
                log::info!("[+] Settings icon texture loaded");
            } else {
                log::info!("[!] Settings icon data not available");
            }
        }
    }

    {
        let params = DEBUG_PARAMS.lock().unwrap();
        let needs_rerender = params.changed;
        if TEXT_TEXTURE.lock().unwrap().is_none() || needs_rerender {
            let (tex, size) = text_rendering::render_text_texture(
                egui_ctx,
                "PLAY (PEACOCK)",
                params.font_height,
                params.font_weight,
                params.text_width,
                params.text_height,
            );
            *TEXT_TEXTURE.lock().unwrap() = tex;
            *TEXT_SIZE.lock().unwrap() = size;
            drop(params);
            let mut params_mut = DEBUG_PARAMS.lock().unwrap();
            params_mut.changed = false;
        }
    }

    {
        let version = PEACOCK_VERSION.lock().unwrap().clone();
        if let Some(version) = version {
            let params = DEBUG_PARAMS.lock().unwrap();
            let needs_rerender = params.version_changed;
            let mut version_texture = PEACOCK_VERSION_TEXTURE.lock().unwrap();
            if version_texture.is_none() || needs_rerender {
                let (tex, size) = text_rendering::render_text_texture(
                    egui_ctx,
                    &version,
                    params.version_font_height,
                    params.version_font_weight,
                    params.version_text_width,
                    params.version_text_height,
                );
                *version_texture = tex;
                *PEACOCK_VERSION_SIZE.lock().unwrap() = size;
                drop(params);
                let mut params_mut = DEBUG_PARAMS.lock().unwrap();
                params_mut.version_changed = false;
            }
        }
    }

    {
        if ICON_TEXTURE.lock().unwrap().is_some()
            && TEXT_TEXTURE.lock().unwrap().is_some()
            && SETTINGS_ICON_TEXTURE.lock().unwrap().is_some()
        {
            *UI_READY.lock().unwrap() = true;
        }
    }

    egui::CentralPanel::default()
        .frame(egui::Frame {
            inner_margin: egui::Margin::ZERO,
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
            corner_radius: egui::CornerRadius::ZERO,
            shadow: egui::Shadow::NONE,
            ..Default::default()
        })
        .show(egui_ctx, |ui| {
            let button_rect = egui::Rect::from_min_size(
                egui::pos2(9.0, *HEIGHT.lock().unwrap() - 35.0),
                egui::vec2(25.0, 25.0),
            );
            let mut icon_hovered = false;
            if let Some(icon_tex) = &*SETTINGS_ICON_TEXTURE.lock().unwrap() {
                let response = ui.allocate_rect(button_rect, egui::Sense::click());
                if response.hovered() {
                    ui.painter().rect_filled(
                        button_rect,
                        egui::CornerRadius::ZERO,
                        egui::Color32::from_rgba_premultiplied(100, 100, 100, 150),
                    );
                }
                ui.put(
                    button_rect,
                    egui::Image::new((icon_tex.id(), egui::vec2(25.0, 25.0))),
                );
                if response.clicked() {
                    *SHOULD_SPAWN_EXTRA.lock().unwrap() = true;
                }
                icon_hovered = response.hovered();
            }

            let button_rect = egui::Rect::from_min_size(
                egui::pos2(
                    crate::constants::WINDOW_POS_X,
                    crate::constants::WINDOW_POS_Y,
                ),
                egui::vec2(
                    crate::constants::WINDOW_WIDTH,
                    crate::constants::WINDOW_HEIGHT,
                ),
            );
            let response = ui.allocate_rect(button_rect, egui::Sense::click());
            if let Some(icon_tex) = &*ICON_TEXTURE.lock().unwrap() {
                let icon_rect = egui::Rect::from_min_size(
                    button_rect.min
                        + egui::vec2(crate::constants::ICON_POS_X, crate::constants::ICON_POS_Y),
                    egui::vec2(25.0, 25.0),
                );
                let disabled = *PLAY_BUTTON_DISABLED.lock().unwrap();
                let icon_tint = if disabled {
                    egui::Color32::GRAY
                } else {
                    egui::Color32::WHITE
                };
                ui.put(
                    icon_rect,
                    egui::Image::new((icon_tex.id(), egui::vec2(25.0, 25.0))).tint(icon_tint),
                );
            }
            let disabled = *PLAY_BUTTON_DISABLED.lock().unwrap();
            let text_color = if disabled {
                egui::Color32::GRAY
            } else if response.hovered() {
                egui::Color32::from_rgb(138, 138, 138)
            } else {
                egui::Color32::WHITE
            };
            if let Some(text_tex) = &*TEXT_TEXTURE.lock().unwrap() {
                let params = crate::gui::debug::DEBUG_PARAMS.lock().unwrap();
                let text_rect = egui::Rect::from_min_size(
                    button_rect.min + egui::vec2(params.text_pos_x, params.text_pos_y),
                    *TEXT_SIZE.lock().unwrap(),
                );
                ui.put(
                    text_rect,
                    egui::Image::new((text_tex.id(), *TEXT_SIZE.lock().unwrap())).tint(text_color),
                );
            }
            if response.clicked() {
                if *PLAY_BUTTON_DISABLED.lock().unwrap() {
                    return;
                }
                *PLAY_BUTTON_DISABLED.lock().unwrap() = true;
                perform_injection();
            }
            let play_hovered = response.hovered();
            *crate::gui::overlay::INPUT_NEEDED.lock().unwrap() = icon_hovered || play_hovered;

            if let Some(version_tex) = &*PEACOCK_VERSION_TEXTURE.lock().unwrap() {
                let params = crate::gui::debug::DEBUG_PARAMS.lock().unwrap();
                let version_rect = egui::Rect::from_min_size(
                    egui::pos2(params.version_text_pos_x, params.version_text_pos_y),
                    *PEACOCK_VERSION_SIZE.lock().unwrap(),
                );
                ui.put(
                    version_rect,
                    egui::Image::new((version_tex.id(), *PEACOCK_VERSION_SIZE.lock().unwrap()))
                        .tint(egui::Color32::WHITE),
                );
            }
        });
}
