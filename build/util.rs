use std::env;
use std::fs::{File, DirEntry};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde_json::Value;

pub const LOCALES_DIR: &str = "locales";

pub fn get_output_file(path: &str) -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    Path::new(&out_dir).join(path)
}

pub fn read_json(path: &str) -> Value {
    let file = File::open(path).unwrap_or_else(|e| panic!("Couldn't open file '{path}': {e}"));
    let reader = BufReader::new(file);
    
    serde_json::from_reader(reader).unwrap_or_else(|e| panic!("Invalid JSON in file '{path}': {e}"))
}

pub fn get_locale_files() -> impl Iterator<Item = DirEntry> {
    Path::new(LOCALES_DIR)
        .read_dir()
        .unwrap_or_else(|e| panic!("Couldn't read directory '{LOCALES_DIR}': {e}"))
        .flatten()
}
