use eframe::egui::Ui;

use crate::{gui::{GdSfx, self}, library::LibraryEntry, settings};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {    
    match entry {
        LibraryEntry::Category { children, .. } => {
            for child in children {
                render(ui, gdsfx, child);
            }
        }
        LibraryEntry::Sound { id, .. } => {
            if settings::has_favourite(id) {
                gui::add_sfx_button(ui, gdsfx, entry);
            }
        }
    }
}
