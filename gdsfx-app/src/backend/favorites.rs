use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use gdsfx_files::paths;
use gdsfx_library::EntryId;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

static FAVORITES_FILE: Lazy<PathBuf> = Lazy::new(|| {
    paths::runtime::PROJECT_DIRS.config_local_dir()
        .join("favorites.json")
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorites(HashSet<EntryId>);

impl Default for Favorites {
    fn default() -> Self {
        let mut favorites = HashSet::new();
        favorites.insert(4451); // FIRE IN THE HOLE
        Self(favorites)
    }
}

impl Favorites {
    pub fn load_or_default() -> Self {
        gdsfx_files::read_json_file(&*FAVORITES_FILE).unwrap_or_default()
    }

    fn try_save(&self) -> Result<()> {
        let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");

        gdsfx_files::create_parent_dirs(&*FAVORITES_FILE)?;
        gdsfx_files::write_file(&*FAVORITES_FILE, json_data)?;

        Ok(())
    }

    pub fn has_favorite(&self, id: EntryId) -> bool {
        self.0.contains(&id)
    }

    pub fn toggle_favorite(&mut self, id: EntryId) {
        // clippy says i shouldnt use boolean short circuiting :(
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
