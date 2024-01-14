use gdsfx_audio::AudioSettings;
use gdsfx_library::LibraryEntry;

use crate::tabs::Tab;

use settings::Settings;

use self::search::SearchSettings;

pub mod favorites;
pub mod settings;
pub mod search;

pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,

    pub settings: Settings,
    pub search_settings: SearchSettings,
    pub audio_settings: AudioSettings,
}

impl AppState {
    pub fn load() -> Self {
        let settings = Settings::load_or_default();
        rust_i18n::set_locale(&settings.locale);

        Self {
            selected_tab: Tab::default(),
            selected_sfx: None,

            settings,
            search_settings: SearchSettings::default(),
            audio_settings: AudioSettings::default(),
        }
    }
}
