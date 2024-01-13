use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{GdSfx, layout};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    layout::add_search_area(ui, gdsfx);
    render_recursive(ui, gdsfx, gdsfx.library.get_root().clone());
}

fn render_recursive(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    match &entry.kind {
        EntryKind::Category { children } => {
            for &child in children {
                render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone());
            }
        },
        EntryKind::Sound { bytes, duration } => {
            // if settings::has_favourite(id) {
            //     gui::add_sfx_button(ui, gdsfx, entry);
            // }
        },
    }
}
