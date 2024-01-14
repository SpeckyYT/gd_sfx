use gdsfx_audio::AudioSettings;
use gdsfx_library::LibraryEntry;

use crate::{tabs::Tab, sorting::Sorting, settings::Settings};

pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,
    pub search_query: String,
    pub sorting_mode: Sorting,

    pub settings: Settings,
    pub audio_settings: AudioSettings,
}

impl AppState {
    pub fn load() -> Self {
        let settings = Settings::load_or_default();
        rust_i18n::set_locale(&settings.locale);

        Self {
            selected_tab: Tab::default(),
            selected_sfx: None,
            search_query: String::new(),
            sorting_mode: Sorting::default(),

            settings,
            audio_settings: AudioSettings::default(),
        }
    }
}
