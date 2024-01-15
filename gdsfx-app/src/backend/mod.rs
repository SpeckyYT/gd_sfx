use std::{sync::Arc, thread, collections::HashMap};

use eframe::epaint::mutex::Mutex;
use favorites::Favorites;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{LibraryEntry, EntryId, EntryKind, Library};
use search::SearchSettings;
use settings::PersistentSettings;

use crate::tabs::Tab;

pub mod favorites;
pub mod settings;
pub mod search;

type SfxCache = HashMap<EntryId, Vec<u8>>;

#[derive(Default)]
pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,

    pub search_settings: SearchSettings,
    pub audio_settings: AudioSettings,

    matching_entries_cache: HashMap<EntryId, bool>,
    sfx_cache: Arc<Mutex<SfxCache>>,
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

    pub fn update(&mut self) {
        self.matching_entries_cache.clear();
    }

    pub fn is_matching_entry(&mut self, entry: &LibraryEntry, library: &Library) -> bool {
        if let Some(&matching) = self.matching_entries_cache.get(&entry.id) {
            return matching
        }

        let matching = match &entry.kind {
            EntryKind::Category => {
                library
                    .get_children(entry)
                    .any(|child| self.is_matching_entry(child, library))
            }

            EntryKind::Sound { .. } => {
                let search = self.search_settings.search_query.to_lowercase();

                // TODO: stats system for storing which files have been downloaded
                (!self.search_settings.show_downloaded /* || entry.file_exists() */)
                    && entry.name.to_lowercase().contains(&search)
                    || entry.id.to_string() == search
            }
        };

        self.matching_entries_cache.insert(entry.id, matching);
        matching
    }

    pub fn play_sound(&self, entry: &LibraryEntry, app_state: &AppState) {
        let file_handler = entry.create_file_handler(&app_state.settings.gd_folder);
        let cache = self.sfx_cache.clone();
        let audio_settings = app_state.audio_settings;

        thread::spawn(move || {
            let bytes = file_handler.try_get_bytes(&mut cache.lock());
            if let Some(bytes) = bytes {
                gdsfx_audio::stop_all();
                gdsfx_audio::play_sound(bytes, audio_settings);
            }
        });
    }

    pub fn download_sound(&self, entry: &LibraryEntry, app_state: &AppState) {
        let file_handler = entry.create_file_handler(&app_state.settings.gd_folder);
        let cache = self.sfx_cache.clone();

        thread::spawn(move || file_handler.try_store_bytes(&mut cache.lock()));
    }
}
