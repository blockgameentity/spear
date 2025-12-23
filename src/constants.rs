#[allow(dead_code)]
use dirs;
use lazy_static::lazy_static;
use std::path::PathBuf;

pub const PLAY_ICON_HASH: &str = "d934764398d1bf4f74f21a6cbb2a621a3763e31f84ed733c7192c8897c40ae2d";
pub const SETTINGS_ICON_HASH: &str =
    "e21238ae2a7a43e4c424f47aa544e8d47952abb071d0c50dccba9be22db22a66";

pub const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
pub const DWMWCP_DONOTROUND: u32 = 1;
pub const WINDOW_WIDTH: f32 = 222.0;
pub const WINDOW_HEIGHT: f32 = 39.0;
pub const WINDOW_POS_X: f32 = 79.0;
pub const WINDOW_POS_Y: f32 = 89.0;
pub const ICON_POS_X: f32 = 9.0;
pub const ICON_POS_Y: f32 = 7.0;

pub const TARGET_WIDTH: u32 = 608;
pub const TARGET_HEIGHT: u32 = 344;

lazy_static! {
    pub static ref SPEAR_PATH: PathBuf = dirs::data_local_dir().unwrap().join("spear");
}
