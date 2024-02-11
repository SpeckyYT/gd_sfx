use std::{fs, path::Path};

use gdsfx_files::paths;


#[cfg(target_os = "windows")]
const INCLUDE_DIR: &str = "include/windows";
#[cfg(target_os = "linux")]
const INCLUDE_DIR: &str = "include/linux";
#[cfg(target_os = "macos")]
const INCLUDE_DIR: &str = "include/macos";

fn main() {
    let target_dir = paths::build::get_dynamic_library_dir().expect("No dynamic library directory found");

    for file in gdsfx_files::read_dir(INCLUDE_DIR).unwrap() {
        let source = file.path();
        let destination = Path::new(target_dir).join(file.file_name());
        fs::copy(&source, &destination).unwrap();

        gdsfx_build::cargo_rerun_if_changed(source);
        gdsfx_build::cargo_rerun_if_changed(destination);
    }
}
