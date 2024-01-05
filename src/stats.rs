use std::{sync::{Arc, Mutex}, thread::{spawn, JoinHandle}};

use eframe::epaint::ahash::HashSet;
use lazy_static::lazy_static;

use crate::util::GD_FOLDER;

lazy_static!{
    pub static ref EXISTING_SOUND_FILES: Arc<Mutex<HashSet<i64>>> = Default::default();
}

pub fn add_file_to_stats(id: i64) {
    EXISTING_SOUND_FILES.lock().unwrap().insert(id);
}

pub fn remove_file_from_stats(id: i64) {
    EXISTING_SOUND_FILES.lock().unwrap().remove(&id);
}

pub fn check_all_sfx_files() -> JoinHandle<()> {
    spawn(|| {
        if let Ok(readdir) = GD_FOLDER.read_dir() {
            for file in readdir.flatten() {
                let path = file.path();

                let string = path.file_name().unwrap().to_str().unwrap();

                if string.starts_with('s') && string.ends_with(".ogg") {
                    let sliced = &string[1..string.len()-4];
                    let parsed = sliced.parse().unwrap();
                    add_file_to_stats(parsed);
                }
            }
        }
    })
}
