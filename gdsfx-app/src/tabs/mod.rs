use strum::EnumIter;

mod library;
mod favorites;
mod tools;
mod settings;
mod stats;
mod credits;

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
