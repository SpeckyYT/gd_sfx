use eframe::egui::Image;
use strum::EnumIter;

use crate::{localized_enum, images};

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
            Self::Library => images::MAGNIFYING_GLASS,
            Self::Favorites => images::STAR_SOLID,
            Self::Tools => images::TOOLS,
            Self::Stats => images::CHART,
            Self::Settings => images::GEAR,
            Self::Credits => images::PEOPLE_GROUP,
        }
        .into()
    }
}
