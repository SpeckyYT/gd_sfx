use std::sync::Arc;

use eframe::epaint::{ahash::HashSet, mutex::Mutex};
use lazy_static::lazy_static;

use crate::{util::GD_FOLDER, library::LibraryEntry};

pub mod gui;

pub struct Stats {
    pub bytes: u128,
    pub duration: u128,
    pub files: i64,
}

lazy_static!{
    pub static ref EXISTING_SOUND_FILES: Arc<Mutex<HashSet<i64>>> = Default::default();
}

pub fn add_file_to_stats(id: i64) {
    EXISTING_SOUND_FILES.lock().insert(id);
}

pub fn remove_file_from_stats(id: i64) {
    EXISTING_SOUND_FILES.lock().remove(&id);
}

pub fn add_existing_sfx_files() {
    if let Ok(readdir) = GD_FOLDER.read_dir() {
        readdir.flatten()
            .map(|file| file.file_name().into_string().unwrap())
            .filter(|s| s.starts_with('s') && s.ends_with(".ogg"))
            .map(|s| s[1..s.len()-4].parse::<i64>().unwrap())
            .for_each(add_file_to_stats);
    }
}

pub fn get_sound_stats(entry: &LibraryEntry) -> Stats {
    match entry {
        LibraryEntry::Category { children, .. } => children
            .iter()
            .map(get_sound_stats)
            .reduce(|a, b| Stats {
                bytes: a.bytes + b.bytes,
                duration: a.duration + b.duration,
                files: a.files + b.files
            })
            .unwrap_or(Stats { bytes: 0, duration: 0, files: 1 }),

        LibraryEntry::Sound { bytes, duration, .. } => Stats {
            bytes: *bytes as u128,
            duration: *duration as u128,
            files: 1
        }
    }
}
