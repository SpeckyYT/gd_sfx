pub mod top_panel;
pub mod left_window;
pub mod right_window;
use top_panel::*;
use left_window::*;
use right_window::*;

use eframe::{
    egui::Context,
    NativeOptions,
};

use crate::library::{Library, LibraryEntry};

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

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        top_panel(ctx, self);
        left_window(ctx, self);
        right_window(ctx, self.selected_sfx.as_ref());
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

