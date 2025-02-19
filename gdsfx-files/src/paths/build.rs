//! Paths that are relevant for building the project.
//! See .cargo/config.toml for environment variables

pub const CARGO_WORKSPACE_ROOT: &str = env!("CARGO_WORKSPACE_ROOT");

pub const LIBS_SOURCE_DIR: &str = env!("GDSFX_LIBS_SOURCE_DIR");
pub const LIBS_TARGET_DIR: &str = env!("GDSFX_LIBS_TARGET_DIR");

pub const PROJECT_SETTINGS: &str = env!("GDSFX_PROJECT_SETTINGS_FILE");
pub const LOCALE_SCHEMA_TARGET: &str = env!("GDSFX_LOCALE_SCHEMA_TARGET_FILE");
pub const LOCALE_SCHEMA_SOURCE: &str = env!("GDSFX_LOCALE_SCHEMA_SOURCE_FILE");
pub const LOCALES_DIR: &str = env!("GDSFX_LOCALES_DIR");
pub const CREDITS: &str = env!("GDSFX_CREDITS_FILE");
pub const THEME_CREDITS: &str = env!("GDSFX_THEME_CREDITS_FILE");
