use strum::EnumIter;

pub mod library;
pub mod favorites;
pub mod tools;
pub mod settings;
pub mod stats;
pub mod credits;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Tab {
    #[default]
    Library,
    Favourites,
    Tools,
    Settings,
    Stats,
    Credits,
}

impl Tab {
    pub fn get_localized_name(&self) -> String {
        t!(match self {
            Tab::Library => "tab.library",
            Tab::Favourites => "tab.favorites",
            Tab::Tools => "tab.tools",
            Tab::Settings => "tab.settings",
            Tab::Stats => "tab.stats",
            Tab::Credits => "tab.credits",
        })
    }
}
