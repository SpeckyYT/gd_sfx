fn main() {
    set_libs_target_dir()
}

/// https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths
/// determined in buildscript to get actual OS rather than target OS
fn set_libs_target_dir() {
    #[allow(unused_assignments)]
    let mut path = None;

    #[cfg(windows)] {
        path = env!("PATH").split(';').next();
    }
    #[cfg(target_os = "macos")] {
        path = env!("DYLD_FALLBACK_LIBRARY_PATH").split(':').next();
    }
    #[cfg(unix)] {
        path = env!("LD_LIBRARY_PATH").split(':').next();
    }

    if let Some(path) = path {
        build_script::cargo_rustc_env("GDSFX_LIBS_TARGET_DIR", path);
    }
}
