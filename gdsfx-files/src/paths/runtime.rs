use std::{path::PathBuf, env};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub const APP_NAME: &str = "GDSFX";

pub static PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("one", "Specky", APP_NAME)
        .expect("No home directory found")
});

pub static GD_FOLDER: Lazy<Option<PathBuf>> = Lazy::new(|| {
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
