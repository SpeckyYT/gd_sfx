use std::{ops::Range, path::PathBuf, sync::Arc};

use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Serialize, Deserialize};

use crate::paths;

lazy_static! {
    static ref SETTINGS_FILE: PathBuf = paths::runtime::PROJECT_DIRS.config_local_dir().join("settings.json");

    pub static ref SETTINGS: Arc<Mutex<Settings>> = Default::default();
    // used to check for modifications
    static ref LAST_SAVE: Arc<Mutex<Settings>> = Default::default();
}

pub(super) fn initialize() {
    let settings: Settings = gdsfx_data::read_json_file(&*SETTINGS_FILE)
        .unwrap_or_else(|_| Default::default());

    *SETTINGS.lock() = settings.clone();
    *LAST_SAVE.lock() = settings;
}

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub gd_folder: Option<PathBuf>,
    
    pub locale: String,

    pub hide_empty_categories: bool,
    
    pub download_ids_range: Range<u32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gd_folder: paths::runtime::try_get_gd_folder(),

            locale: String::from("en_US"),
            
            hide_empty_categories: false,
            
            download_ids_range: 0..14500,
        }
    }
}

impl Settings {
    pub fn try_save_if_changed(&self) -> Result<()> {
        let mut last_save = LAST_SAVE.lock();

        if *self != *last_save {
            let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");
            gdsfx_data::write_file(&*SETTINGS_FILE, json_data)?;

            *last_save = self.clone();
        }
        
        Ok(())
    }
}
