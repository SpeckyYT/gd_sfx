// paths that are relevant for building the project
// see .cargo/config.toml for environment variables

pub const CARGO_WORKSPACE_ROOT: &str = env!("CARGO_WORKSPACE_ROOT");

pub const PROJECT_SETTINGS_FILE: &str = env!("GDSFX_PROJECT_SETTINGS_FILE");
pub const LOCALE_SCHEMA_TARGET_FILE: &str = env!("GDSFX_LOCALE_SCHEMA_TARGET_FILE");
pub const LOCALE_SCHEMA_SOURCE_FILE: &str = env!("GDSFX_LOCALE_SCHEMA_SOURCE_FILE");
pub const LOCALES_DIR: &str = env!("GDSFX_LOCALES_DIR");
pub const CREDITS_FILE: &str = env!("GDSFX_CREDITS_FILE");
