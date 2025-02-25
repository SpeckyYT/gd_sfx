use std::path::PathBuf;
use std::sync::LazyLock;
use ahash::HashSet;
use anyhow::Result;
use serde::{Serialize, Deserialize};

use gdsfx_library::EntryId;

static FAVORITES_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    gdsfx_shared::paths::PROJECT_DIR.config_local_dir().join("favorites.json")
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorites(HashSet<EntryId>);

impl Default for Favorites {
    fn default() -> Self {
        Self(HashSet::from_iter([4451]))
    }
}

impl Favorites {
    pub fn load() -> Self {
        gdsfx_files::read_json(&*FAVORITES_FILE).unwrap_or_default()
    }

    fn try_save(&self) -> Result<()> {
        let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");

        let _ = gdsfx_files::create_parent_dirs(&*FAVORITES_FILE);
        gdsfx_files::write_file(&*FAVORITES_FILE, json_data)
    }

    pub fn has_favorite(&self, id: EntryId) -> bool {
        self.0.contains(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = EntryId> + use<'_> {
        self.0.iter().copied()
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
