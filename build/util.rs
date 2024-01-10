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

// specky wtf is this
// lets make a new crate for stuff that doesnt depend on build/ or runtime stuff
// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

// temporary, copy-pasted from src/util/mod.rs
pub fn geometry_dash_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
            PathBuf::from(env::var("localappdata").expect("No local app data"))
                .join("GeometryDash")
    } else if cfg!(target_os = "macos") {
            PathBuf::from(env::var("HOME").expect("No home directory"))
                .join("Library/Application Support/GeometryDash")
    } else if cfg!(target_os = "linux") {
        let home_path_str: String = env::var("HOME").expect("home directory ENV variable not set");
        let home_path: &Path = Path::new(home_path_str.as_str());

        let possible_gd_paths: [&str; 2] = [
            ".steam/steam/steamapps/compatiata/322170/drive_c/users/steamuser/Local Settings/Application Data/GeometryDash",
            "PortWINE/PortProton/prefixes/DEFAULT/drive_c/users/steamuser/AppData/Local/GeometryDash"
        ];

        for path in possible_gd_paths {
            let full_path: PathBuf = home_path.join(path);
            if full_path.exists() {
                return full_path;
            }
        }

        panic!("no GD path found");
    } else if cfg!(target_os = "android") {
        PathBuf::from("/data/data/com.robtopx.geometryjump")
    } else {
        panic!("Unsupported operating system");
    }
}
