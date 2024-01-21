use std::{fs, path::Path};

const INCLUDE_DIR: &str = "include/";
const TARGET_DIR: &str = "../target/debug/deps/";

fn main() {
    for file in gdsfx_files::read_dir(INCLUDE_DIR).unwrap() {
        let source = file.path();
        let destination = Path::new(TARGET_DIR).join(file.file_name());
        fs::copy(&source, &destination).unwrap();

        gdsfx_build::cargo_rerun_if_changed(source);
        gdsfx_build::cargo_rerun_if_changed(destination);
    }
}
