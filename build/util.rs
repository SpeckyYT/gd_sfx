use std::{env, fs};
use std::fs::{File, DirEntry};
use std::io::BufReader;
use std::path::Path;

use serde_json::Value;

pub const LOCALES_DIR: &str = "locales";

pub fn write_output_file(path: impl AsRef<Path>, contents: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);

    fs::write(&path, contents).unwrap_or_else(|e| panic!("Couldn't write to file {path:?}: {e}"));
}

pub fn read_json(path: impl AsRef<Path>) -> Value {
    let path = path.as_ref();
    let file = File::open(path).unwrap_or_else(|e| panic!("Couldn't open file {path:?}: {e}"));
    let reader = BufReader::new(file);
    
    serde_json::from_reader(reader).unwrap_or_else(|e| panic!("Invalid JSON in file {path:?}: {e}"))
}

pub fn get_locale_files() -> impl Iterator<Item = DirEntry> {
    let path = Path::new(LOCALES_DIR);

    path.read_dir()
        .unwrap_or_else(|e| panic!("Couldn't read directory {path:?}: {e}"))
        .flatten()
}
