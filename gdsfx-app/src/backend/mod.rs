use std::{thread, sync::Arc, collections::HashMap, path::Path};

use eframe::epaint::mutex::Mutex;
use favorites::Favorites;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{Library, LibraryEntry, EntryId, EntryKind, FileEntry};
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
                (!self.search_settings.show_downloaded /* || FileEntry::new(entry.id).file_exists(&self.settings.gd_folder) */ )
                    && entry.name.to_lowercase().contains(&search)
                    || entry.id.to_string() == search
            }
        }
    }

    pub fn is_gd_folder_valid(&self) -> bool {
        let path = Path::new(&self.settings.gd_folder);
        path.is_absolute() && path.is_dir()
    }

    pub fn play_sound(&self, id: EntryId) {
        let cache = self.sfx_cache.clone();
        let gd_folder = self.settings.gd_folder.clone();
        let audio_settings = self.audio_settings;

        thread::spawn(move || {
            let bytes = {
                let mut cache = cache.lock();
                cache.get(&id).cloned().or_else(|| {
                    let file_entry = FileEntry::new(id);
                    let bytes = file_entry.try_read_bytes(gd_folder)
                        .or_else(|| file_entry.try_download_bytes());

                    if let Some(bytes) = bytes.as_ref() {
                        cache.insert(id, bytes.clone());
                    }

                    bytes
                })
            };

            if let Some(bytes) = bytes {
                gdsfx_audio::stop_all();
                gdsfx_audio::play_sound(bytes, audio_settings);
            }
        });
    }

    pub fn download_sound(&self, id: EntryId) {
        if !self.is_gd_folder_valid() { return }

        let file_entry = FileEntry::new(id);
        let gd_folder = &self.settings.gd_folder;

        if file_entry.file_exists(gd_folder) { return }

        let cache = self.sfx_cache.clone();
        let gd_folder = gd_folder.clone();

        thread::spawn(move || {
            let bytes = cache.lock().get(&id).cloned()
                .or_else(|| file_entry.try_download_bytes());

            if let Some(bytes) = bytes {
                file_entry.try_write_bytes(gd_folder, bytes);
            }
        });
    }
}
