use std::{path::PathBuf, sync::{Arc, Mutex}, fs};

use eframe::epaint::ahash::HashSet;
use lazy_static::lazy_static;

use crate::{util::GD_FOLDER, encoding::{zlib_decode, base64_decode, zlib_encode, base64_encode}};

lazy_static!{
    pub static ref FAVOURITES_FILE: PathBuf = GD_FOLDER.join("gdsfx_favourites.dat");
    pub static ref FAVOURITES_LIST: Arc<Mutex<HashSet<i64>>> = Arc::new(Mutex::new(read_file()));

    pub static ref EMPTY_FAVOURITES: String = base64_encode(&zlib_encode(&[])); 
}

pub fn read_file() -> HashSet<i64> {
    if FAVOURITES_FILE.exists() {
        let mut favourites = HashSet::default();

        let data = fs::read(FAVOURITES_FILE.as_path()).unwrap();

        let data = base64_decode(&data);
        let data = zlib_decode(&data);

        let string = std::str::from_utf8(&data).unwrap_or("");

        string.split(',').for_each(|line| {
            if let Ok(int) = line.parse() {
                favourites.insert(int);
            }
        });

        favourites
    } else {
        fs::write(FAVOURITES_FILE.as_path(), EMPTY_FAVOURITES.as_str()).unwrap();
        HashSet::default()
    }
}

pub fn save() {
    let string = FAVOURITES_LIST.lock().unwrap().iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    let data = zlib_encode(string.as_bytes());
    let data = base64_encode(&data);
    fs::write(FAVOURITES_FILE.as_path(), data).unwrap();
}

pub fn add_favourite(id: i64) {
    FAVOURITES_LIST.lock().unwrap().insert(id);
    save();
}

pub fn has_favourite(id: i64) -> bool {
    FAVOURITES_LIST.lock().unwrap().contains(&id)
}

pub fn remove_favourite(id: i64) {
    FAVOURITES_LIST.lock().unwrap().remove(&id);
}
