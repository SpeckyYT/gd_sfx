use eframe::egui::{Context, TopBottomPanel};
use strum::{IntoEnumIterator, EnumIter};

use super::GdSfx;


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

pub fn top_panel(ctx: &Context, gdsfx: &mut GdSfx) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            Tab::iter().for_each(|tab| {
                ui.selectable_value(&mut gdsfx.tab, tab, tab.get_localized_name());
            });
        });
        ui.add_space(2.0);
    });
}
