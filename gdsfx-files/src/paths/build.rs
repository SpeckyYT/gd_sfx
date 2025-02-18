//! Paths that are relevant for building the project.
//! See .cargo/config.toml for environment variables

pub const CARGO_WORKSPACE_ROOT: &str = env!("CARGO_WORKSPACE_ROOT");

/// https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths
pub fn get_dynamic_library_dir() -> Option<&'static str> {
    #[cfg(debug_assertions)]
    #[cfg(windows)]
    return env!("PATH").split(';').next();

    #[cfg(debug_assertions)]
    #[cfg(unix)]
    return env!("LD_LIBRARY_PATH").split(':').next();

    #[cfg(debug_assertions)]
    #[cfg(target_os = "macos")]
    return env!("DYLD_FALLBACK_LIBRARY_PATH").split(':').next();

    #[allow(unreachable_code)]
    None
}

pub const PROJECT_SETTINGS_FILE: &str = env!("GDSFX_PROJECT_SETTINGS_FILE");
pub const LOCALE_SCHEMA_TARGET_FILE: &str = env!("GDSFX_LOCALE_SCHEMA_TARGET_FILE");
pub const LOCALE_SCHEMA_SOURCE_FILE: &str = env!("GDSFX_LOCALE_SCHEMA_SOURCE_FILE");
pub const LOCALES_DIR: &str = env!("GDSFX_LOCALES_DIR");
pub const CREDITS_FILE: &str = env!("GDSFX_CREDITS_FILE");
pub const THEME_CREDITS: &str = env!("GDSFX_THEME_CREDITS_FILE");
