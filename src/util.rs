use std::path::PathBuf;
use std::{env, fs};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use eframe::epaint::ahash::HashMap;
use eframe::epaint::mutex::Mutex;
use lazy_static::lazy_static;
use rayon::prelude::*;

use crate::gui::left_window::DOWNLOAD_PROGRESS;
use crate::library::LibraryEntry;
use crate::stats::{add_existing_sfx_files, EXISTING_SOUND_FILES};

pub const LIBRARY_WIDTH: f32 = 200.0;
pub const RIGHT_PANEL_WIDTH: f32 = 500.0;
pub const TOTAL_WIDTH: f32 = LIBRARY_WIDTH + RIGHT_PANEL_WIDTH;

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

    pub static ref LOCAL_SFX_LIBRARY: Arc<Mutex<HashMap<i64, Vec<u8>>>> = Default::default();
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

pub fn format_locale(locale: &str) -> String {
    let name = t!("language.name", locale = locale);
    let region = t!("language.region", locale = locale);
    
    if region.is_empty() {
        name
    } else {
        format!("{name} ({region})")
    }
}

pub fn download_everything(library: LibraryEntry) -> JoinHandle<()> {
    spawn(|| {
        fn recursive(library: LibraryEntry) -> Vec<LibraryEntry> {
            match library {
                LibraryEntry::Category { children, .. } =>
                    children.into_iter().flat_map(recursive).collect(),
                LibraryEntry::Sound { .. } =>
                    vec![library],
            }
        }
        let all_sfx = recursive(library);

        DOWNLOAD_PROGRESS.lock().unwrap().1 = all_sfx.len();

        all_sfx.into_par_iter()
        .for_each(|entry| {
            entry.download_and_store();
            DOWNLOAD_PROGRESS.lock().unwrap().0 += 1;
        });
    })
}

pub fn delete_everything() -> JoinHandle<()> {
    spawn(|| {
        let _ = add_existing_sfx_files();
        let existing_sound_file = EXISTING_SOUND_FILES.lock().unwrap();
        
        DOWNLOAD_PROGRESS.lock().unwrap().1 = existing_sound_file.len();
        
        existing_sound_file.iter()
        .for_each(|id| {
            let filename = format!("s{id}.ogg");
            let filepath = GD_FOLDER.join(filename);

            if filepath.exists() {
                let _ = fs::remove_file(filepath);
            }

            DOWNLOAD_PROGRESS.lock().unwrap().0 += 1;    
        });

    })
}
