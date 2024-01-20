use eframe::egui::Image;
use strum::EnumIter;

use crate::{localized_enum, i18n::LocalizedEnum};

pub mod library;
pub mod favorites;
pub mod tools;
pub mod settings;
pub mod stats;
pub mod credits;

localized_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
    pub enum Tab = "tab" {
        #[default]
        Library = "library",
        Favorites = "favorites",
        Tools = "tools",
        Settings = "settings",
        Stats = "stats",
        Credits = "credits",
    }
}

impl Tab {
    pub fn icon(&self) -> Image<'static> {
        let bytes: &[u8] = match self {
            Self::Library => include_bytes!("../../../assets/wtf_is_this.png"),
            Self::Credits => include_bytes!("../../../assets/wtf_is_this.png"),
            Self::Favorites => include_bytes!("../../../assets/favorite.png"),
            Self::Settings => include_bytes!("../../../assets/settings.png"),
            Self::Stats => include_bytes!("../../../assets/statistics.png"),
            Self::Tools => include_bytes!("../../../assets/informations.png"),
        };

        Image::from_bytes(self.localize_variant().to_string(), bytes)
    }
}
