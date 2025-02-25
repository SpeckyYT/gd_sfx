use std::{env, fs, path::Path};

use anyhow::Context;

fn get_source_dir() -> impl AsRef<Path> {
    // cfg target flags in buildscript determine actual OS rather than target OS
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    Path::new(gdsfx_shared::paths::LIBS_DIR).join(build::cargo_cfg_target_os())
}

fn get_target_dir() -> Option<String> {
    // cfg target flags in buildscript determine actual OS rather than target OS
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths

    if cfg!(windows) {
        return env::var("PATH").ok()?.split(';').next().map(str::to_owned)
    }

    if cfg!(target_os = "macos") {
        return env::var("DYLD_FALLBACK_LIBRARY_PATH").ok()?.split(':').next().map(str::to_owned)
    }

    if cfg!(unix) {
        return env::var("LD_LIBRARY_PATH").ok()?.split(':').next().map(str::to_owned)
    }

    None
}

pub fn build() {
    let source_dir = get_source_dir();
    let target_dir = get_target_dir().expect("No dynamic library directory found");

    for file in gdsfx_files::read_dir(source_dir).unwrap() {
        let source = file.path();
        let target = Path::new(&target_dir).join(file.file_name());

        fs::copy(&source, &target)
            .with_context(|| format!("Failed to copy {} to {}", source.display(), target.display()))
            .unwrap();

        build::rerun_if_changed(source);
        build::rerun_if_changed(target);
    }

    build::rustc_link_search(target_dir);
}
