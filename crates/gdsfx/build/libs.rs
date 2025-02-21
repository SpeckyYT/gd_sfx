use std::{env, fs, path::Path};

use anyhow::Context;

fn get_source_dir() -> impl AsRef<Path> {
    // cfg target flags in buildscript determine actual OS rather than target OS
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    Path::new(files::paths::LIBS_DIR).join(target_os)
}

fn get_target_dir() -> Option<&'static str> {
    // cfg target flags in buildscript determine actual OS rather than target OS
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths

    #[cfg(windows)] {
        return env!("PATH").split(';').next();
    }
    #[cfg(target_os = "macos")] {
        return env!("DYLD_FALLBACK_LIBRARY_PATH").split(':').next();
    }
    #[cfg(unix)] {
        return env!("LD_LIBRARY_PATH").split(':').next();
    }

    #[allow(unreachable_code)]
    None
}

pub fn build() {
    let source_dir = get_source_dir();
    let target_dir = get_target_dir().expect("No dynamic library directory found");

    for file in files::read_dir(source_dir).unwrap() {
        let source = file.path();
        let target = Path::new(target_dir).join(file.file_name());

        fs::copy(&source, &target)
            .with_context(|| format!("Failed to copy {} to {}", source.display(), target.display()))
            .unwrap();

        build::cargo_rerun_if_changed(source);
        build::cargo_rerun_if_changed(target);
    }

    build::cargo_rustc_link_search(target_dir);
}
