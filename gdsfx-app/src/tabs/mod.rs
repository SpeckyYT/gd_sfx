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
            {$($pat:pat => $image:expr $(=> rgb($r:expr, $g:expr, $b:expr))?),* $(,)?} => {
                {
                    let image: Image<'static> = match self {
                        $(
                            $pat => $image,
                        )*
                    }.into();
                    let image = match self {
                        $(
                            $pat => image.tint(Color32::GRAY)
                                $( .tint(Color32::from_rgb($r, $g, $b)) )?,
                        )*
                    };
                    image
                }
            };
        }

        images!{
            Self::Library => images::MAGNIFYING_GLASS,
            Self::Favorites => images::STAR_SOLID,
            Self::Tools => images::TOOLS,
            Self::Settings => images::GEAR,
            Self::Stats => images::CHART,
            Self::Credits => images::PEOPLE_GROUP,
        }
    }
}
