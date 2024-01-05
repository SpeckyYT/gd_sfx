use std::{sync::{Arc, Mutex}, io};

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

pub fn add_existing_sfx_files() -> Result<(), io::Error> {
    GD_FOLDER.read_dir()?
        .flatten()
        .map(|file| file.file_name().into_string().unwrap())
        .filter(|s| s.starts_with('s') && s.ends_with(".ogg"))
        .map(|s| s[1..s.len()-4].parse::<i64>().unwrap())
        .for_each(add_file_to_stats);

    Ok(())
}
