use std::{path::PathBuf, env, sync::Arc};

use eframe::epaint::{ahash::{HashMap, HashSet}, mutex::Mutex};
use lazy_static::lazy_static;

pub mod encoding;
pub mod requests;

pub const MIN_LIBRARY_WIDTH: f32 = 200.0;
pub const DEFAULT_LIBRARY_WIDTH: f32 = 300.0;
pub const RIGHT_PANEL_WIDTH: f32 = 500.0;
pub const TOTAL_WIDTH: f32 = DEFAULT_LIBRARY_WIDTH + RIGHT_PANEL_WIDTH;

// → see build/sfx_list.rs
include!(concat!(env!("OUT_DIR"), "/sfx_list.rs"));

pub const TOTAL_HEIGHT: f32 = 600.0;

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
    pub static ref LOCAL_SFX_LIBRARY: Arc<Mutex<HashMap<u32, Vec<u8>>>> = Default::default();
    pub static ref UNLISTED_SFX: Arc<Mutex<HashSet<u32>>> = Default::default();
}

pub fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}

pub fn stringify_duration(centiseconds: i64) -> String {
    format!("{:.2}", centiseconds as f64 / 100.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stringify_duration() {
        assert_eq!("0.00",  stringify_duration(0));
        assert_eq!("0.12",  stringify_duration(12));
        assert_eq!("3.45",  stringify_duration(345));
        assert_eq!("67.89", stringify_duration(6789));
        
        assert_eq!("0.01",  stringify_duration(1));
        assert_eq!("0.10",  stringify_duration(10));
        assert_eq!("1.00",  stringify_duration(100));
    }
}
