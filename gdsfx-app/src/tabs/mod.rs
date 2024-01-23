use eframe::egui::Image;
use strum::EnumIter;
use crate::epaint::Color32;

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
        macro_rules! images {
            {$($pat:pat => $image:expr, rgb($r:expr, $g:expr, $b:expr)),* $(,)?} => {
                {
                    let image: Image<'static> = match self {
                        $($pat => $image,)*
                    }.into();
                    match self {
                        $($pat => image.tint(Color32::from_rgb($r, $g, $b)),)*
                    }
                }
            };
        }

        images!{
            Self::Library => images::MAGNIFYING_GLASS, rgb(16,99,188),
            Self::Favorites => images::STAR_SOLID, rgb(255,196,70),
            Self::Tools => images::TOOLS, rgb(123,100,72),
            Self::Settings => images::GEAR, rgb(191,191,191),
            Self::Stats => images::CHART, rgb(105,219,61),
            Self::Credits => images::PEOPLE_GROUP, rgb(255, 165, 64),
        }
    }
}
