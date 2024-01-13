use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{GdSfx, layout};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    render_recursive(ui, gdsfx, gdsfx.library.get_root().clone());
}

fn render_recursive(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    match entry.kind {
        EntryKind::Category { children } => {
            // TODO extra shit
            for child in children {
                render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone());
            }
        }

        EntryKind::Sound { .. } => layout::add_sfx_button(ui, gdsfx, entry),
    }
}
