use std::{collections::HashMap, sync::Arc, thread, cell::RefCell};

use eframe::epaint::mutex::Mutex;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{Library, EntryId, LibraryEntry, EntryKind};

type SfxCache = HashMap<EntryId, Vec<u8>>;

pub struct LibraryManager {
    pub library: Library,
    sfx_cache: Arc<Mutex<SfxCache>>,
    matching_entries: RefCell<HashMap<EntryId, bool>>,
}

impl LibraryManager {
    pub fn load() -> Self {
        Self {
            library: gdsfx_library::load_library(),
            sfx_cache: Default::default(),
            matching_entries: Default::default(),
        }
    }

    pub fn is_matching_entry(&self, entry: &LibraryEntry, search_query: &str) -> bool {
        if let Some(&enabled) = self.matching_entries.borrow().get(&entry.id) {
            return enabled
        }

        let enabled = match &entry.kind {
            EntryKind::Category => {
                self.library
                    .get_children(entry)
                    .any(|child| self.is_matching_entry(child, search_query))
            },
    
            EntryKind::Sound { .. } => {
                let search = search_query.to_lowercase();
                entry.name.to_lowercase().contains(&search) || entry.id.to_string() == search
            }
        };

        self.matching_entries.borrow_mut().insert(entry.id, enabled);
        enabled
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
