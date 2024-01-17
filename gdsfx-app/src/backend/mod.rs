use std::{thread, sync::Arc, collections::{HashMap, HashSet}, path::Path, fs};

use eframe::epaint::mutex::Mutex;
use favorites::Favorites;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{Library, LibraryEntry, EntryId, EntryKind, FileEntry};
use search::SearchSettings;
use settings::PersistentSettings;
use tools::ToolProgress;

use crate::tabs::Tab;

pub mod favorites;
pub mod settings;
pub mod search;
pub mod tools;

#[derive(Default)]
pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,

    pub search_settings: SearchSettings,
    pub audio_settings: AudioSettings,

    pub tool_progress: Arc<Mutex<Option<ToolProgress>>>,
    pub download_id_range: (EntryId, EntryId),

    downloaded_sfx: Arc<Mutex<HashSet<EntryId>>>,
    sfx_cache: Arc<Mutex<HashMap<EntryId, Vec<u8>>>>,
}

impl AppState {
    pub fn load() -> Self {
        let settings = PersistentSettings::load_or_default();
        rust_i18n::set_locale(&settings.locale);


        let downloaded_sfx = fs::read_dir(&settings.gd_folder)
            .map(|read_dir| {
                read_dir
                    .flatten()
                    .flat_map(|file| file.file_name().into_string())
                    .filter(|file_name| file_name.starts_with('s') && file_name.ends_with(".ogg"))
                    .flat_map(|file_name| file_name[1..file_name.len()-4].parse())
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        Self {
            settings,
            favorites: Favorites::load_or_default(),
            download_id_range: (0, 14500),
            downloaded_sfx: Arc::new(Mutex::new(downloaded_sfx)),
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

                (!self.search_settings.show_downloaded || self.downloaded_sfx.lock().contains(&entry.id))
                    && entry.name.to_lowercase().contains(&search)
                    || entry.id.to_string() == search
            }
        }
    }

    pub fn is_gd_folder_valid(&self) -> bool {
        let path = Path::new(&self.settings.gd_folder);
        path.is_absolute() && path.is_dir()
    }

    pub fn is_sfx_downloaded(&self, id: EntryId) -> bool {
        self.downloaded_sfx.lock().contains(&id)
    }

    pub fn play_sfx(&self, id: EntryId) {
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

    pub fn download_sfx(&self, id: EntryId) {
        if !self.is_gd_folder_valid() { return }

        let file_entry = FileEntry::new(id);
        let gd_folder = &self.settings.gd_folder;

        if file_entry.file_exists(gd_folder) { return }

        let cache = self.sfx_cache.clone();
        let gd_folder = gd_folder.clone();
        let downloaded_sfx = self.downloaded_sfx.clone();

        thread::spawn(move || {
            let bytes = cache.lock().get(&id).cloned()
                .or_else(|| file_entry.try_download_bytes());

            let Some(bytes) = bytes else { return };
            if file_entry.try_write_bytes(gd_folder, bytes).is_ok() {
                downloaded_sfx.lock().insert(id);
            }
        });
    }

    pub fn delete_sfx(&self, id: EntryId) {
        if FileEntry::new(id).try_delete_file(&self.settings.gd_folder).is_ok() {
            self.downloaded_sfx.lock().remove(&id);
        }
    }

    pub fn get_sfx_count(&self) -> usize {
        self.downloaded_sfx.lock().len()
    }
}
