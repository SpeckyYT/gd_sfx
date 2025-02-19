use std::{env, fs, path::Path};

fn main() {
    // cfg! flags aren't set for buildscripts yet
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let source_dir = Path::new(gdsfx_files::paths::build::LIBS_SOURCE_DIR).join(target_os);

    let target_dir = gdsfx_files::paths::build::get_libs_target_dir()
        .expect("No dynamic library directory found");

    for file in gdsfx_files::read_dir(source_dir).unwrap() {
        let source = file.path();
        let destination = Path::new(target_dir).join(file.file_name());
        let _ = fs::copy(&source, &destination);

        build_script::cargo_rerun_if_changed(source);
        build_script::cargo_rerun_if_changed(destination);
    }
}
