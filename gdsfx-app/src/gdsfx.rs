use std::{collections::HashMap, thread, sync::Arc, io::Cursor, time::Instant};

use eframe::{egui::{self, IconData}, epaint::mutex::Mutex};
use gdsfx_audio::AudioSettings;
use gdsfx_data::paths;
use gdsfx_library::{sorting::Sorting, Library, LibraryEntry, EntryId, EntryKind};

use crate::{tabs::Tab, settings::Settings, layout};

type SfxCache = HashMap<EntryId, Vec<u8>>;

const NORMAL_ICON: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/normal.bin"));

pub struct GdSfx {
    pub selected_tab: Tab,

    pub library: Library,
    pub selected_sfx: Option<LibraryEntry>,
    sfx_cache: Arc<Mutex<SfxCache>>,

    pub search_query: String,
    pub sorting: Sorting,
    matching_entries: HashMap<EntryId, bool>,

    pub settings: Settings,
    pub audio_settings: AudioSettings,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;
        
        top_panel::render(self, ctx);
        left_window::render(self, ctx);
        right_window::render(self, ctx);

        self.matching_entries.clear();
    }
}

impl GdSfx {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some(layout::DEFAULT_WINDOW_SIZE),
                min_inner_size: Some(layout::DEFAULT_WINDOW_SIZE * layout::MIN_SCALE_FACTOR),
                resizable: Some(true),
                icon: Some(Arc::new(IconData {
                    rgba: NORMAL_ICON.to_vec(),
                    width: 256,
                    height: 256,
                })),

                ..Default::default()
            },
            follow_system_theme: false,
            default_theme: eframe::Theme::Dark,
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,

            ..Default::default()
        };
        
        eframe::run_native(paths::runtime::APP_NAME, options, Box::new(Self::load))
    }

    fn load(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
        let settings = Settings::load_or_default();
        rust_i18n::set_locale(&settings.locale);

        Box::new(Self {
            selected_tab: Tab::default(),

            library: gdsfx_library::load_library(),
            selected_sfx: None,
            sfx_cache: Default::default(),

            search_query: String::new(),
            sorting: Sorting::default(),
            matching_entries: HashMap::new(),

            settings,
            audio_settings: AudioSettings::default(),
        })
    }

    pub fn is_matching_entry(&mut self, entry: LibraryEntry) -> bool {
        if let Some(&enabled) = self.matching_entries.get(&entry.id) {
            return enabled
        }

        let enabled = match &entry.kind {
            EntryKind::Category { children } => {
                children.iter().any(|&child| {
                    self.is_matching_entry(self.library.get_entry(child).clone())
                })
            },
    
            EntryKind::Sound { .. } => {
                let search = self.search_query.to_lowercase();
                entry.name.to_lowercase().contains(&search) || entry.id.to_string() == search
            }
        };

        self.matching_entries.insert(entry.id, enabled);
        enabled
    }

    pub fn play_sound(&mut self, entry: &LibraryEntry) {
        let audio_settings = self.audio_settings;
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

    pub fn download_sound(&mut self, entry: &LibraryEntry) {
        let cache = self.sfx_cache.clone();
        let entry = entry.clone();
        thread::spawn(move || entry.try_store_bytes(&mut cache.lock()));
    }
}
