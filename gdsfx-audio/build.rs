use std::{fs, path::Path};

#[cfg(debug_assertions)]
fn main() {
    #[cfg(target_os = "windows")]
    const INCLUDE_DIR: &str = "include/windows";
    #[cfg(target_os = "linux")]
    const INCLUDE_DIR: &str = "include/linux";
    #[cfg(target_os = "macos")]
    const INCLUDE_DIR: &str = "include/macos";

    let target_dir = gdsfx_files::paths::build::get_dynamic_library_dir()
        .expect("No dynamic library directory found");

    for file in gdsfx_files::read_dir(INCLUDE_DIR).unwrap() {
        let source = file.path();
        let destination = Path::new(target_dir).join(file.file_name());
        let _ = fs::copy(&source, &destination);

        build_script::cargo_rerun_if_changed(source);
        build_script::cargo_rerun_if_changed(destination);
    }

    build_script::cargo_rustc_link_search(target_dir);
}

#[cfg(not(debug_assertions))]
fn main() {}
