use std::path::PathBuf;
use std::env;
use std::sync::Arc;
use eframe::epaint::ahash::HashMap;
use eframe::epaint::mutex::Mutex;
use lazy_static::lazy_static;

pub const LIBRARY_WIDTH: f32 = 400.0;
pub const RIGHT_PANEL_WIDTH: f32 = 300.0;
pub const TOTAL_WIDTH: f32 = LIBRARY_WIDTH + RIGHT_PANEL_WIDTH;

pub const TOTAL_HEIGHT: f32 = 200.0;

lazy_static!{
    pub static ref APPDATA_FOLDER: PathBuf = PathBuf::from(env::var("localappdata").unwrap());
    pub static ref GD_FOLDER: PathBuf = APPDATA_FOLDER.join("GeometryDash");
    pub static ref SFX_LIBRARY_FILE: PathBuf = GD_FOLDER.join("sfxlibrary.dat");

    pub static ref LOCAL_SFX_LIBRARY: Arc<Mutex<HashMap<i64, Vec<u8>>>> = Default::default();
}

pub fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}
