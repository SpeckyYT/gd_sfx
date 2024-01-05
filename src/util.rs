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
    pub static ref GD_FOLDER: PathBuf = {
        if cfg!(target_os = "windows") {
            PathBuf::from(env::var("localappdata").expect("No local app data"))
                .join("GeometryDash")
        } else if cfg!(target_os = "macos") {
            PathBuf::from(env::var("HOME").expect("No home directory"))
                .join("Library/Application Support/GeometryDash")
        } else if cfg!(target_os = "linux") {
            PathBuf::from(env::var("HOME").expect("No home directory"))
                .join(".steam/steam/steamapps/compatdata/322170/pfx/drive_c/users/steamuser/Local Settings/Application Data/GeometryDash")
        } else if cfg!(target_os = "android") {
            PathBuf::from("/data/data/com.robtopx.geometryjump")
        } else {
            panic!("Unsupported operating system");
        }
    };
    pub static ref SFX_LIBRARY_FILE: PathBuf = GD_FOLDER.join("sfxlibrary.dat");

    pub static ref LOCAL_SFX_LIBRARY: Arc<Mutex<HashMap<i64, Vec<u8>>>> = Default::default();
}

pub fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}

pub fn stringify_duration(duration: i64) -> String {
    let mut centiseconds = format!("{:>03}", duration);
    centiseconds.insert(centiseconds.len() - 2, '.');
    centiseconds
}
