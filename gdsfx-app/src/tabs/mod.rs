use eframe::egui::{Image, include_image};
use strum::EnumIter;

use crate::localized_enum;

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
        match self {
            Self::Library => include_image!("../../../assets/svg/magnifying-glass-solid.svg"),
            Self::Favorites => include_image!("../../../assets/svg/star-solid.svg"),
            Self::Tools => include_image!("../../../assets/svg/screwdriver-wrench-solid.svg"),
            Self::Stats => include_image!("../../../assets/svg/chart-simple-solid.svg"),
            Self::Settings => include_image!("../../../assets/svg/gear-solid.svg"),
            Self::Credits => include_image!("../../../assets/svg/people-group-solid.svg"),
        }
        .into()
    }
}
