use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone)]
pub struct DebugParams {
    pub font_height: i32,
    pub font_weight: i32,
    pub text_width: f32,
    pub text_height: f32,
    pub text_pos_x: f32,
    pub text_pos_y: f32,
    pub changed: bool,
    pub version_font_height: i32,
    pub version_font_weight: i32,
    pub version_text_width: f32,
    pub version_text_height: f32,
    pub version_text_pos_x: f32,
    pub version_text_pos_y: f32,
    pub version_changed: bool,
}

lazy_static! {
    pub static ref DEBUG_PARAMS: Mutex<DebugParams> = Mutex::new(DebugParams {
        font_height: 30,
        font_weight: 400,
        text_width: 150.0,
        text_height: 21.0,
        text_pos_x: 45.0,
        text_pos_y: 9.0,
        changed: true,
        version_font_height: 30,
        version_font_weight: 400,
        version_text_width: 27.0,
        version_text_height: 15.0,
        version_text_pos_x: 575.0,
        version_text_pos_y: 315.0,
        version_changed: true,
    });
}
