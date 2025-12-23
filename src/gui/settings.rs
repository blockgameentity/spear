use crate::config::peacock::{PeacockCategory, load_peacock_config, save_peacock_config};
use crate::config::spear::{SpearConfig, load_spear_config, save_spear_config};
use crate::gui::overlay_utils::{capitalize_first, format_option_key};

use eframe::EventLoopBuilderHook;
use egui;
use lazy_static::lazy_static;
use std::sync::Mutex;
use winit::platform::windows::EventLoopBuilderExtWindows;

#[derive(Clone)]
struct SettingsState {
    spear_config: SpearConfig,
    peacock_categories: Vec<PeacockCategory>,
    dirty: bool,
}

lazy_static! {
    static ref STATE: Mutex<Option<SettingsState>> = Mutex::new(None);
}

pub fn show_settings_window() {
    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        event_loop_builder.with_any_thread(true);
    }));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(600.0, 800.0)),
        event_loop_builder,
        ..Default::default()
    };
    eframe::run_simple_native("Settings", options, |ctx, _frame| {
        ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Settings");
                ui.separator();

                {
                    let mut state_guard = STATE.lock().unwrap();
                    if state_guard.is_none() {
                        *state_guard = Some(SettingsState {
                            spear_config: load_spear_config(),
                            peacock_categories: load_peacock_config(),
                            dirty: false,
                        });
                    }
                }
                if let Some(state) = STATE.lock().unwrap().as_mut() {
                    ui.label("Peacock GitHub");
                    let response =
                        ui.text_edit_singleline(&mut state.spear_config.peacock_github_repo);
                    if response.changed() {
                        log::info!(
                            "[+] Peacock GitHub changed to: {}",
                            state.spear_config.peacock_github_repo
                        );
                        state.dirty = true;
                    }

                    for cat in state.peacock_categories.iter_mut() {
                        ui.separator();
                        ui.heading(&capitalize_first(&cat.name));
                        for opt in cat.options.iter_mut() {
                            if let Some(poss_vals) = &opt.possible_values {
                                ui.label(&format_option_key(&opt.key));
                                let mut selected = opt.value.clone();
                                egui::ComboBox::from_id_salt(&opt.key)
                                    .selected_text(&selected)
                                    .show_ui(ui, |ui| {
                                        for val in poss_vals {
                                            ui.selectable_value(
                                                &mut selected,
                                                val.clone(),
                                                val.as_str(),
                                            );
                                        }
                                    });
                                if selected != opt.value {
                                    opt.value = selected;
                                    log::info!("[+] Option {} changed to: {}", opt.key, opt.value);
                                    state.dirty = true;
                                }
                            } else if opt.value == "true" || opt.value == "false" {
                                let mut checked = opt.value == "true";
                                if ui
                                    .checkbox(&mut checked, &format_option_key(&opt.key))
                                    .changed()
                                {
                                    opt.value = if checked { "true" } else { "false" }.to_string();
                                    log::info!("[+] Option {} changed to: {}", opt.key, opt.value);
                                    state.dirty = true;
                                }
                            } else {
                                ui.label(&format_option_key(&opt.key));
                                let response = ui.text_edit_singleline(&mut opt.value);
                                if response.changed() {
                                    log::info!("[+] Option {} changed to: {}", opt.key, opt.value);
                                    state.dirty = true;
                                }
                            }
                            ui.label(&opt.description);
                        }
                    }
                    #[cfg(debug_assertions)]
                    {
                        ui.separator();
                        ui.heading("Debug Parameters");
                        let mut params = crate::gui::debug::DEBUG_PARAMS.lock().unwrap();
                        ui.label("Font Height");
                        if ui
                            .add(egui::DragValue::new(&mut params.font_height))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Font Weight");
                        if ui
                            .add(egui::DragValue::new(&mut params.font_weight))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Text Width");
                        if ui
                            .add(egui::DragValue::new(&mut params.text_width))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Text Height");
                        if ui
                            .add(egui::DragValue::new(&mut params.text_height))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Text Pos X");
                        if ui
                            .add(egui::DragValue::new(&mut params.text_pos_x))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Text Pos Y");
                        if ui
                            .add(egui::DragValue::new(&mut params.text_pos_y))
                            .changed()
                        {
                            params.changed = true;
                        }
                        ui.label("Version Font Height");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_font_height))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                        ui.label("Version Font Weight");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_font_weight))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                        ui.label("Version Text Width");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_text_width))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                        ui.label("Version Text Height");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_text_height))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                        ui.label("Version Text Pos X");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_text_pos_x))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                        ui.label("Version Text Pos Y");
                        if ui
                            .add(egui::DragValue::new(&mut params.version_text_pos_y))
                            .changed()
                        {
                            params.version_changed = true;
                        }
                    }
                    if state.dirty {
                        log::info!("[+] Saving configs");
                        save_spear_config(&state.spear_config);
                        save_peacock_config(&state.peacock_categories);
                        state.dirty = false;
                    }
                } else {
                    log::info!("[!] Settings state not initialized");
                }
            });
        });
    })
    .unwrap();
}
