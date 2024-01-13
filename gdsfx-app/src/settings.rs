use std::{ops::Range, path::PathBuf};

use anyhow::Result;
use educe::Educe;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

use crate::paths;

static SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
    paths::runtime::PROJECT_DIRS.config_local_dir()
        .join("settings.json")
});

#[derive(Educe, Serialize, Deserialize)]
#[educe(Default, Clone, PartialEq)]
pub struct Settings {
    #[educe(Default = paths::runtime::GD_FOLDER.clone())]
    pub gd_folder: Option<PathBuf>,

    #[educe(Default = String::from("en_US"))]
    pub locale: String,

    #[educe(Default = false)]
    pub hide_empty_categories: bool,

    #[educe(Default = 0..14500)]
    pub download_ids_range: Range<u32>,

    #[serde(skip)]
    #[educe(Clone(method(ignore_option)), PartialEq(ignore))]
    last_save: Option<Box<Settings>>,
}

fn ignore_option<T>(_: &Option<T>) -> Option<T> { None }

impl Settings {
    pub fn load_or_default() -> Self {
        gdsfx_data::read_json_file(&*SETTINGS_FILE)
            .unwrap_or_default()
    }

    pub fn try_save_if_changed(&mut self) -> Result<()> {
        if self.has_changed() {
            let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");
            
            gdsfx_data::create_parent_dirs(&*SETTINGS_FILE)?;
            gdsfx_data::write_file(&*SETTINGS_FILE, json_data)?;

            self.last_save = Some(Box::new(self.clone()));
        }
        Ok(())
    }

    fn has_changed(&self) -> bool {
        self.last_save.as_ref()
            .map(|last| self.ne(last))
            .unwrap_or(true) // has not saved before
    }
}
