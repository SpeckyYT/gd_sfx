use std::{path::PathBuf, sync::Arc, fs};

use eframe::epaint::{ahash::HashSet, mutex::Mutex};
use lazy_static::lazy_static;

use crate::util::{GD_FOLDER, encoding::*};

pub mod gui;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Settings {
    pub hide_empty_categories: bool,
}

lazy_static!{
    pub static ref FAVOURITES_FILE: PathBuf = GD_FOLDER.join("gdsfx_favourites.dat");

    pub static ref FIRST_READ: (HashSet<u32>, Settings) = read_settings_file();
    pub static ref FAVOURITES_LIST: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(FIRST_READ.0.clone()));
    pub static ref SETTINGS: Arc<Mutex<Settings>> = Arc::new(Mutex::new(FIRST_READ.1));

    pub static ref EMPTY_FAVOURITES: String = full_encode(&[]); 
}

pub const FAVOURITES_CHARACTER: char = '⭐';

pub fn read_settings_file() -> (HashSet<u32>, Settings) {
    if FAVOURITES_FILE.exists() {
        let mut favourites = HashSet::default();

        let data = fs::read(FAVOURITES_FILE.as_path()).unwrap();
        let data = full_decode(&data);

        let string = std::str::from_utf8(&data).unwrap_or("");

        let mut settings = Settings::default();
        string.split('|')
            .enumerate()
            .for_each(|(i, s)| {
                match i {
                    0 => { // favorites
                        s.split(',').for_each(|line| {
                            if let Ok(int) = line.parse() {
                                favourites.insert(int);
                            }
                        })
                    }
                    1 => settings.hide_empty_categories = s == "true",
                    2 => rust_i18n::set_locale(s),
                    _ => (),
                }
            });
        (favourites, settings)
    } else {
        fs::write(FAVOURITES_FILE.as_path(), EMPTY_FAVOURITES.as_str()).unwrap();
        (HashSet::default(), Settings::default())
    }
}

pub fn generate_save_string() -> String {
    let favourites_string = FAVOURITES_LIST.lock()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",");
    
    let settings = SETTINGS.lock();

    let strings = [
        favourites_string,
        settings.hide_empty_categories.to_string(),
        rust_i18n::locale(),
    ];
    strings.join("|")
}

pub fn save() {
    let full_string = generate_save_string();
    let data = full_encode(full_string.as_bytes());
    fs::write(FAVOURITES_FILE.as_path(), data).unwrap();
}

pub fn add_favourite(id: u32) {
    FAVOURITES_LIST.lock().insert(id);
    save();
}

pub fn has_favourite(id: u32) -> bool {
    FAVOURITES_LIST.lock().contains(&id)
}

pub fn remove_favourite(id: u32) {
    FAVOURITES_LIST.lock().remove(&id);
    save();
}
