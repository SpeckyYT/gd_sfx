use std::{collections::HashMap, sync::Arc, thread};

use eframe::epaint::mutex::Mutex;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{EntryId, EntryKind, Library, LibraryEntry};

use crate::app_state::search::SearchSettings;

type SfxCache = HashMap<EntryId, Vec<u8>>;

pub struct LibraryManager {
    pub library: Library,
    sfx_cache: Arc<Mutex<SfxCache>>,
}

impl LibraryManager {
    pub fn load() -> Self {
        Self {
            library: gdsfx_library::load_library(),
            sfx_cache: Default::default(),
        }
    }

    pub fn is_matching_entry(&self, entry: &LibraryEntry, search_settings: &SearchSettings) -> bool {
        match &entry.kind {
            EntryKind::Category => {
                self.library
                    .get_children(entry)
                    .any(|child| self.is_matching_entry(child, search_settings))
            }

            EntryKind::Sound { .. } => {
                let search = search_settings.search_query.to_lowercase();

                // TODO: stats system for storing which files have been downloaded
                (!search_settings.filter_downloaded /* || entry.file_exists() */)
                    && entry.name.to_lowercase().contains(&search)
                    || entry.id.to_string() == search
            }
        }
    }

    pub fn play_sound(&self, entry: &LibraryEntry, audio_settings: AudioSettings) {
        let cache = self.sfx_cache.clone();
        let entry = entry.clone();

        thread::spawn(move || {
            let bytes = entry.try_get_bytes(&mut cache.lock());
            if let Some(bytes) = bytes {
                gdsfx_audio::stop_all();
                gdsfx_audio::play_sound(bytes, audio_settings);
            }
        });
    }

    pub fn download_sound(&self, entry: &LibraryEntry) {
        let cache = self.sfx_cache.clone();
        let entry = entry.clone();
        thread::spawn(move || entry.try_store_bytes(&mut cache.lock()));
    }
}
