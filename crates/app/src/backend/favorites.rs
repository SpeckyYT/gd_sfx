use std::path::PathBuf;

use ahash::HashSet;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

use library::EntryId;

static FAVORITES_FILE: Lazy<PathBuf> = Lazy::new(|| {
    files::paths::PROJECT_DIR.config_local_dir().join("favorites.json")
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorites(HashSet<EntryId>);

impl Default for Favorites {
    fn default() -> Self {
        Self([4451].into_iter().collect())
    }
}

impl Favorites {
    pub fn load() -> Self {
        files::read_json(&*FAVORITES_FILE).unwrap_or_default()
    }

    fn try_save(&self) -> Result<()> {
        let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");

        let _ = files::create_parent_dirs(&*FAVORITES_FILE);
        files::write_file(&*FAVORITES_FILE, json_data)
    }

    pub fn has_favorite(&self, id: EntryId) -> bool {
        self.0.contains(&id)
    }

    pub fn toggle_favorite(&mut self, id: EntryId) {
        if !self.0.insert(id) {
            self.0.remove(&id);
        }

        if self.try_save().is_err() {
            // undo on failure
            if !self.0.remove(&id) {
                self.0.insert(id);
            }
        }
    }
}
