use gdsfx_audio::AudioSettings;
use gdsfx_library::LibraryEntry;

use crate::tabs::Tab;

use settings::PersistentSettings;

use self::{search::SearchSettings, favorites::Favorites};

pub mod favorites;
pub mod settings;
pub mod search;

pub struct AppState {
    pub selected_tab: Tab,
    pub selected_sfx: Option<LibraryEntry>,

    pub settings: PersistentSettings,
    pub favorites: Favorites,

    pub search_settings: SearchSettings,
    pub audio_settings: AudioSettings,
}

impl AppState {
    pub fn load() -> Self {
        let settings = PersistentSettings::load_or_default();
        rust_i18n::set_locale(&settings.locale);

        Self {
            selected_tab: Tab::default(),
            selected_sfx: None,

            settings,
            favorites: Favorites::load_or_default(),

            search_settings: SearchSettings::default(),
            audio_settings: AudioSettings::default(),
        }
    }
}
