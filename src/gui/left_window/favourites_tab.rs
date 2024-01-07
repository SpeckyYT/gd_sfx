use eframe::egui::Ui;

use crate::{gui::GdSfx, library::LibraryEntry, settings};
use super::add_sfx_button;

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {    
    match entry {
        LibraryEntry::Category { children, .. } => {
            for child in children {
                render(ui, gdsfx, child);
            }
        }
        LibraryEntry::Sound { id, .. } => {
            if settings::has_favourite(id) {
                add_sfx_button(ui, gdsfx, entry);
            }
        }
    }
}
