use std::{thread, sync::Arc, path::Path};

use ahash::{HashMap, HashSet};
use eframe::egui::{self, Visuals};
use educe::Educe;
use itertools::{Either, Itertools};
use parking_lot::{Mutex, RwLock};
use strum::EnumIter;

use audio::{AudioSettings, AudioSystem};
use library::{music, EntryId, FileEntry, FileEntryKind, MusicLibrary, SfxLibrary};
use library::sfx::{EntryKind, SfxLibraryEntry};

use crate::layout;
use crate::{tabs::Tab, localized_enum};

use self::favorites::Favorites;
use self::konami::Konami;
use self::search::{MusicFilters, SearchSettings};
use self::settings::{ColorTheme, PersistentSettings};
use self::tools::ToolProgress;

pub mod favorites;
pub mod settings;
pub mod search;
pub mod tools;
pub mod konami;

#[derive(Educe)]
#[educe(Default)]
pub struct AppState {
    pub selected_tab: Tab,
    pub library_page: LibraryPage, // todo: actually give this a better name
    pub selected_sfx: Option<SfxLibraryEntry>,
    pub selected_music: Option<music::Song>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,
    
    pub search_settings: SearchSettings,
    pub music_filters: MusicFilters,
    pub audio_settings: AudioSettings,

    #[educe(Default = AudioSystem::new().unwrap())]
    pub audio_system: Arc<RwLock<AudioSystem>>,

    pub unlisted_sfx: Vec<EntryId>,
    pub unlisted_music: Vec<EntryId>,

    pub tool_progress: Arc<Mutex<Option<ToolProgress>>>,

    #[educe(Default = (0, 14500))]
    pub download_id_range_sfx: (EntryId, EntryId),
    #[educe(Default = (10000000, 10010000))]
    pub download_id_range_music: (EntryId, EntryId),

    // TODO https://docs.rs/notify/6.1.1/notify/
    // to keep track of externally added and removed SFX?
    downloaded_sfx: Arc<Mutex<HashSet<EntryId>>>,
    sfx_cache: Arc<Mutex<HashMap<EntryId, Vec<u8>>>>,

    downloaded_music: Arc<Mutex<HashSet<EntryId>>>,
    music_cache: Arc<Mutex<HashMap<EntryId, Vec<u8>>>>,

    pub konami: Konami,
}

impl AppState {
    pub fn load(settings: PersistentSettings, sfx_library: &SfxLibrary, music_library: &MusicLibrary) -> Self {
        let (downloaded_sfx, downloaded_music): (HashSet<EntryId>, HashSet<EntryId>) =
            files::read_dir(&settings.gd_folder).into_iter().flatten()
                .flat_map(|file| file.file_name().into_string())
                .filter(|name| name.ends_with(".ogg"))
                .filter_map(|name| {
                    let is_sfx = name.starts_with('s');
                    let id = name[is_sfx as usize..name.len() - 4].parse::<EntryId>().ok()?;
                    Some((is_sfx, id))
                })
                .partition_map(|(is_sfx, id)| match is_sfx {
                    true => Either::Left(id),
                    false => Either::Right(id),
                });

        // TODO how do we want to update unlisted_sfx? a fn register_sfx(&mut self, id: EntryId, library?)
        // and/or can unlisted_sfx be (partially) refactored into gdsfx-library?
        // additional things to consider:
        // - favorites tab
        // - tools (un)registering sfx ids → thread safety
        // - storing unlisted sfx? or only show downloaded ones
        let library_sfx = sfx_library.sound_ids().iter().copied().collect();
        let library_music = music_library.songs.keys().copied().collect();
        let unlisted_sfx = downloaded_sfx.difference(&library_sfx).copied().collect();
        let unlisted_music = downloaded_music.difference(&library_music).copied().collect();

        Self {
            settings,
            favorites: Favorites::load(),
            downloaded_sfx: Arc::new(Mutex::new(downloaded_sfx)),
            downloaded_music: Arc::new(Mutex::new(downloaded_music)),
            unlisted_sfx,
            unlisted_music,
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

    pub fn is_matching_song(&self, song: &music::Song) -> bool {
        if self.search_settings.show_downloaded && !self.is_music_downloaded(song.id) {
            return false
        }

        let search = self.search_settings.search_query.to_lowercase();
        song.name.to_lowercase().contains(&search) || song.id.to_string() == search
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

    pub fn is_music_downloaded(&self, id: EntryId) -> bool {
        self.downloaded_music.lock().contains(&id)
    }

    pub fn play_sound(&self, file_entry: impl FileEntry + 'static) {
        let cache = match file_entry.kind() {
            FileEntryKind::Sound => self.sfx_cache.clone(),
            FileEntryKind::Song => self.music_cache.clone(),
        };
        
        let gd_folder = self.settings.gd_folder.clone();
        let audio_system = Arc::clone(&self.audio_system);

        thread::spawn(move || {
            let bytes = {
                let mut cache = cache.lock();
                cache.get(&file_entry.id()).cloned().or_else(|| {
                    let bytes = file_entry.try_read_bytes(gd_folder)
                        .or_else(|| file_entry.try_download_bytes());

                    if let Some(bytes) = bytes.as_ref() {
                        cache.insert(file_entry.id(), bytes.clone());
                    }

                    bytes
                })
            };

            if let Some(bytes) = bytes {
                let _ = AudioSystem::play_audio(audio_system, &bytes);
            }
        });        
    }

    pub fn download_sound(&self, file_entry: impl FileEntry + 'static) {
        if !self.is_gd_folder_valid() { return }

        let gd_folder = &self.settings.gd_folder;

        if file_entry.file_exists(gd_folder) { return }

        let cache = match file_entry.kind() {
            FileEntryKind::Sound => self.sfx_cache.clone(),
            FileEntryKind::Song => self.music_cache.clone(),
        };
        let downloaded = match file_entry.kind() {
            FileEntryKind::Sound => self.downloaded_sfx.clone(),
            FileEntryKind::Song => self.downloaded_music.clone(),
        };

        let gd_folder = gd_folder.clone();
        let gd_folder = gd_folder.clone();

        thread::spawn(move || {
            let bytes = cache.lock().get(&file_entry.id()).cloned()
                .or_else(|| file_entry.try_download_bytes());

            let Some(bytes) = bytes else { return };
            if file_entry.try_write_bytes(gd_folder, bytes).is_ok() {
                downloaded.lock().insert(file_entry.id());
            }
        });
    }

    pub fn delete_sound(&self, file_entry: impl FileEntry) {
        if file_entry.try_delete_file(&self.settings.gd_folder).is_ok() {
            match file_entry.kind() {
                FileEntryKind::Sound => self.downloaded_sfx.lock(),
                FileEntryKind::Song => self.downloaded_music.lock(),
            }.remove(&file_entry.id());
        }
    }

    pub fn get_sfx_count(&self) -> usize {
        self.downloaded_sfx.lock().len()
    }

    pub fn get_songs_count(&self) -> usize {
        self.downloaded_music.lock().len()
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

pub fn update(ctx: &egui::Context, app_state: &mut AppState) {
    app_state.konami.update(ctx);

    use crate::theme::*;

    ctx.set_visuals(match app_state.settings.theme {
        ColorTheme::Dark => Visuals::dark(),
        ColorTheme::Light => Visuals::light(),
        ColorTheme::Latte => LATTE.to_visuals(ctx),
        ColorTheme::Frappe => FRAPPE.to_visuals(ctx),
        ColorTheme::Macchiato => MACCHIATO.to_visuals(ctx),
        ColorTheme::Mocha => MOCHA.to_visuals(ctx),
    });
}

pub fn request_optional_repaint(ctx: &egui::Context, app_state: &mut AppState) {
    if 
        app_state.tool_progress.lock().is_some()
        || layout::debug_window::DEBUG_MODE.lock().is_some()
    {
        ctx.request_repaint();
    }
}
