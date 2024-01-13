use std::collections::HashMap;

use eframe::{egui, epaint::Vec2};
use gdsfx_audio::AudioSettings;
use gdsfx_data::paths;
use gdsfx_library::{sorting::Sorting, Library, LibraryEntry, EntryId, EntryKind};

use crate::{tabs::Tab, settings::Settings, layout};

pub struct GdSfx {
    pub selected_tab: Tab,

    pub library: Library,
    pub selected_sfx: Option<LibraryEntry>,

    pub search_query: String,
    enabled_entries: HashMap<EntryId, bool>,
    pub sorting: Sorting,

    pub settings: Settings,
    pub audio_settings: AudioSettings,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;
        
        top_panel::render(self, ctx);
        left_window::render(self, ctx);
        right_window::render(self, ctx);

        self.enabled_entries.clear();
    }
}

impl GdSfx {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some(Vec2 { x: 800.0, y: 600.0 }),
                min_inner_size: Some(Vec2 { x: 560.0, y: 420.0 }),
                resizable: Some(true),

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
            search_query: String::new(),
            enabled_entries: HashMap::new(),
            sorting: Sorting::default(),
            settings,
            audio_settings: AudioSettings::default(),
        })
    }

    pub fn is_enabled_entry(&mut self, entry: LibraryEntry) -> bool {
        if let Some(&enabled) = self.enabled_entries.get(&entry.id) {
            return enabled
        }

        let enabled = match &entry.kind {
            EntryKind::Category { children } => {
                children.iter().any(|&child| {
                    self.is_enabled_entry(self.library.get_entry(child).clone())
                })
            },
    
            EntryKind::Sound { .. } => {
                let search = self.search_query.to_ascii_lowercase();
                entry.name.to_lowercase().contains(&search) || entry.id.to_string() == search
            }
        };

        self.enabled_entries.insert(entry.id, enabled);
        enabled
    }
}
