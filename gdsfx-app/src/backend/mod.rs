use std::{thread::{self, JoinHandle}, sync::Arc, collections::HashMap};

use eframe::epaint::mutex::Mutex;
use favorites::Favorites;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{Library, LibraryEntry, EntryId, EntryKind};
use search::SearchSettings;
use settings::PersistentSettings;

use crate::tabs::Tab;

pub mod favorites;
pub mod settings;
pub mod search;
pub mod tools;

#[derive(Default, Clone)]
pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,

    pub search_settings: SearchSettings,
    pub audio_settings: AudioSettings,

    sfx_cache: Arc<Mutex<HashMap<EntryId, Vec<u8>>>>,
}

impl AppState {
    pub fn load() -> Self {
        let settings = PersistentSettings::load_or_default();
        rust_i18n::set_locale(&settings.locale);

        Self {        
            settings,
            favorites: Favorites::load_or_default(),
            ..Default::default()
        }
    }

    pub fn is_matching_entry(&self, entry: &LibraryEntry, library: &Library) -> bool {
        match &entry.kind {
            EntryKind::Category => {
                library
                    .iter_children(entry)
                    .any(|child| self.is_matching_entry(child, library))
            }

            EntryKind::Sound { .. } => {
                let search = self.search_settings.search_query.to_lowercase();

                // TODO: stats system for storing which files have been downloaded
                (!self.search_settings.show_downloaded /* || entry.create_file_handler(&self.settings.gd_folder).file_exists() */)
                    && entry.name.to_lowercase().contains(&search)
                    || entry.id.to_string() == search
            }
        }
    }

    pub fn play_sound(&self, entry: &LibraryEntry) {
        let cache = self.sfx_cache.clone();
        let entry = entry.clone();
        let file_handler = entry.create_file_handler(&self.settings.gd_folder);
        let audio_settings = self.audio_settings;

        thread::spawn(move || {
            let bytes = cache.lock()
                .get(&entry.id).cloned()
                .or_else(|| file_handler.and_then(|file_handler| file_handler.try_read_bytes()))
                .or_else(|| entry.try_get_bytes());

            if let Some(bytes) = bytes {
                gdsfx_audio::stop_all();
                gdsfx_audio::play_sound(bytes, audio_settings);
            }
        });
    }

    pub fn download_sound(&self, entry: &LibraryEntry) -> Option<JoinHandle<()>> {
        let cache = self.sfx_cache.clone();
        let entry = entry.clone();

        entry.create_file_handler(&self.settings.gd_folder)
            .map(|file_handler|
                thread::spawn(move || {
                    file_handler.try_write_bytes(|| {
                        cache.lock().get(&entry.id).cloned()
                            .or_else(|| file_handler.try_read_bytes())
                            .or_else(|| entry.try_get_bytes())
                    })
                })
            )
    }
}
