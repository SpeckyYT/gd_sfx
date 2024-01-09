use eframe::{
    egui::{Context, Ui},
    NativeOptions,
};
use strum::EnumIter;

use crate::{library::{Library, LibraryEntry}, audio, settings};

mod top_panel;
mod left_window;
mod right_window;

pub type VersionType = usize;

#[derive(Debug, Default, Clone)]
pub struct GdSfx {
    pub cdn_url: Option<String>,
    pub sfx_version: Option<VersionType>,
    pub sfx_library: Option<Library>,

    pub tab: Tab,
    pub search_query: String,
    pub sorting: Sorting,
    pub selected_sfx: Option<LibraryEntry>,
}

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    #[default]
    Default,
    NameInc,   // a - z
    NameDec,   // z - a
    LengthInc, // 0.00 - 1.00
    LengthDec, // 1.00 - 0.00
    IdInc,     // 0 - 9
    IdDec,     // 9 - 0
    SizeInc,   // 0kb - 9kb
    SizeDec,   // 9kb - 0kb
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        top_panel::render(self, ctx);
        left_window::render(self, ctx);
        right_window::render(self, ctx);
    }
}

impl GdSfx {
    pub fn run(self, options: NativeOptions) {
        eframe::run_native("GDSFX", options, Box::new(|_cc| Box::new(self))).unwrap()
    }

    pub fn matches_query(&self, entry: &LibraryEntry) -> bool {
        entry.name().to_ascii_lowercase().contains(&self.search_query.to_ascii_lowercase())
            || entry.id().to_string() == self.search_query
    }
}

pub fn add_sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    if !entry.is_enabled() { return }

    let sound = ui.button(entry.pretty_name());

    let entry_selected = sound.hovered();

    if sound.clicked() {
        audio::stop_audio();
        audio::play_sound(&entry);
    }

    sound.context_menu(|ui| {
        if settings::has_favourite(entry.id()) {
            if ui.button(t!("sound.button.favorite.remove")).clicked() {
                settings::remove_favourite(entry.id());
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.favorite.add")).clicked() {
            settings::add_favourite(entry.id());
            ui.close_menu();
        }

        if entry.exists() {
            if ui.button(t!("sound.button.delete")).clicked() {
                entry.delete();
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.download")).clicked() {
            entry.download_and_store();
            ui.close_menu();
        }
    });

    if entry_selected {
        gdsfx.selected_sfx = Some(entry);
    }
}
