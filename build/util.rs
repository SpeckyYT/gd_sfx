use std::{env, fs};
use std::fs::{File, DirEntry};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use serde::de::DeserializeOwned;

pub const LOCALES_DIR: &str = "locales";

pub fn get_locale_files() -> impl Iterator<Item = DirEntry> {
    let path = Path::new(LOCALES_DIR);

    path.read_dir()
        .unwrap_or_else(|e| panic!("Couldn't read directory {path:?}: {e}"))
        .flatten()
}

pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> T {
    let path = path.as_ref();
    let file = File::open(path).unwrap_or_else(|e| panic!("Couldn't open file {path:?}: {e}"));
    let reader = BufReader::new(file);
    
    serde_json::from_reader::<_, T>(reader)
        .unwrap_or_else(|e| panic!("Incorrect JSON in file {path:?}: {e}"))
}

pub fn write_output_file(path: impl AsRef<Path>, tokens: TokenStream) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    
    // write unformatted token stream first...
    fs::write(&path, tokens.to_string())
        .unwrap_or_else(|e| panic!("Couldn't write to file {path:?}: {e}"));

    // ...so that it can be inspected if parsing the token stream fails...
    let parsed = syn::parse2(tokens)
        .unwrap_or_else(|e| panic!("Couldn't parse token stream for {path:?}: {e}"));
    
    // ...before finally writing the formatted version to the file
    fs::write(&path, prettyplease::unparse(&parsed))
        .unwrap_or_else(|e| panic!("Couldn't write to file {path:?}: {e}"))
}

pub fn geometry_dash_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("localappdata"));
    path.push("GeometryDash");
    path
}
