use std::{thread, sync::Arc, path::Path, fs};
use ahash::{HashMap, HashSet};

use eframe::epaint::mutex::Mutex;
use favorites::Favorites;
use gdsfx_audio::AudioSettings;
use gdsfx_library::{SfxLibrary, EntryId, SfxFileEntry};
use gdsfx_library::sfx::{EntryKind, SfxLibraryEntry};
use search::SearchSettings;
use settings::PersistentSettings;
use tools::ToolProgress;
use strum::EnumIter;

use crate::{tabs::Tab, localized_enum};

use self::search::MusicFilters;

pub mod favorites;
pub mod settings;
pub mod search;
pub mod tools;

#[derive(Default)]
pub struct AppState {
    pub selected_tab: Tab,
    pub library_page: LibraryPage, // todo: actually give this a better name
    pub selected_sfx: Option<SfxLibraryEntry>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,

    pub search_settings: SearchSettings,
    pub music_filters: MusicFilters,
    pub audio_settings: AudioSettings,

    pub unlisted_sfx: Vec<EntryId>,

    pub tool_progress: Arc<Mutex<Option<ToolProgress>>>,
    pub download_id_range: (EntryId, EntryId),

    // TODO https://docs.rs/notify/6.1.1/notify/
    // to keep track of externally added and removed SFX?
    downloaded_sfx: Arc<Mutex<HashSet<EntryId>>>,
    sfx_cache: Arc<Mutex<HashMap<EntryId, Vec<u8>>>>,
}

impl AppState {
    pub fn load(settings: PersistentSettings, sfx_library: &SfxLibrary) -> Self {
        let downloaded_sfx: HashSet<EntryId> = fs::read_dir(&settings.gd_folder)
            .map(|read_dir| {
                read_dir
                    .flatten()
                    .flat_map(|file| file.file_name().into_string())
                    .filter(|file_name| file_name.starts_with('s') && file_name.ends_with(".ogg"))
                    .flat_map(|file_name| file_name[1..file_name.len()-4].parse())
                    .collect()
            })
            .unwrap_or_default();

        // TODO how do we want to update unlisted_sfx? a fn register_sfx(&mut self, id: EntryId, library?)
        // and/or can unlisted_sfx be (partially) refactored into gdsfx-library?
        // additional things to consider:
        // - favorites tab
        // - tools (un)registering sfx ids → thread safety
        // - storing unlisted sfx? or only show downloaded ones
        let library_sfx = &sfx_library.sound_ids().iter().copied().collect();
        let unlisted_sfx = downloaded_sfx.difference(library_sfx).copied().collect();

        Self {
            settings,
            favorites: Favorites::load_or_default(),
            download_id_range: (0, 14500),
            downloaded_sfx: Arc::new(Mutex::new(downloaded_sfx)),
            unlisted_sfx,
            ..Default::default()
        }
    }

    pub fn is_matching_entry(&self, entry: &SfxLibraryEntry, sfx_library: &SfxLibrary) -> bool {
        match &entry.kind {
            EntryKind::Category => {
                sfx_library
                    .iter_children(entry)
                    .any(|child| self.is_matching_entry(child, sfx_library))
            }

            EntryKind::Sound { .. } => {
                if self.search_settings.show_downloaded && !self.is_sfx_downloaded(entry.id) {
                    return false
                }

                let search = self.search_settings.search_query.to_lowercase();
                entry.name.to_lowercase().contains(&search) || entry.id.to_string() == search
            }
        }
    }

    pub fn is_gd_folder_valid(&self) -> bool {
        let path = Path::new(&self.settings.gd_folder);
        path.is_absolute() && path.is_dir()
    }

    pub fn is_tool_running(&self) -> bool {
        self.tool_progress.lock().is_some()
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
                    let file_entry = SfxFileEntry::new(id);
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

        let file_entry = SfxFileEntry::new(id);
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
        if SfxFileEntry::new(id).try_delete_file(&self.settings.gd_folder).is_ok() {
            self.downloaded_sfx.lock().remove(&id);
        }
    }

    pub fn get_sfx_count(&self) -> usize {
        self.downloaded_sfx.lock().len()
    }
}

localized_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
    pub enum LibraryPage = "library_page" {
        #[default]
        Sfx = "sfx",
        Music = "music",
    }
}
