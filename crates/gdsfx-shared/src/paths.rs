use std::{env, path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;

/// environment variable set in `.cargo/config.toml` to determine paths relative to workspace
#[macro_export]
macro_rules! workspace_path {
    ($path:expr) => {
        ::std::concat!(::std::env!("CARGO_WORKSPACE_ROOT"), $path)
    };
}

pub const LIBS_DIR: &str = workspace_path!("libs");
pub const LOCALES_DIR: &str = workspace_path!("locales");

pub static PROJECT_DIR: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("one", "Specky", crate::consts::APP_NAME)
        .expect("No home directory found")
});

pub static DEFAULT_GD_DIR: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    if cfg!(target_os = "windows") {
        return Some(PathBuf::from(&env::var_os("localappdata")?).join("GeometryDash"))
    }

    if cfg!(target_os = "macos") {
        return Some(PathBuf::from(&env::var_os("HOME")?).join("SfxLibrary/Application Support/GeometryDash"))
    }

    if cfg!(target_os = "linux") {
        let home_path = PathBuf::from(&env::var_os("HOME")?);

        let possible_paths = [
            ".steam/steam/steamapps/compatiata/322170/drive_c/users/steamuser/Local Settings/Application Data/GeometryDash",
            "PortWINE/PortProton/prefixes/DEFAULT/drive_c/users/steamuser/AppData/Local/GeometryDash"
        ];

        return possible_paths.iter()
            .map(|path| home_path.join(path))
            .find(|path| path.exists())
    }

    if cfg!(target_os = "android") {
        return Some(PathBuf::from("/data/data/com.robtopx.geometryjump"))
    }

    None
});
