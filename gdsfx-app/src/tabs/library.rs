use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{GdSfx, layout, settings::SearchFilterMode};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    layout::add_search_area(ui, gdsfx);

    let root = gdsfx.library.get_root().clone();
    if let EntryKind::Category { children } = root.kind {
        for child in children {
            render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone())
        }
    }

    // TODO do unlisted fuckery
}

fn render_recursive(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    match entry.kind {
        EntryKind::Category { ref children } => {
            let has_hide_filter = gdsfx.settings.search_filter_mode == SearchFilterMode::Hide;
            let is_enabled = gdsfx.is_enabled_entry(entry.clone());
            
            if is_enabled || has_hide_filter {
                ui.add_enabled_ui(is_enabled, |ui| {
                    ui.collapsing(&entry.name, |ui| {
                        for &child in children {
                            render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone());
                        }
                    })
                });
            }
        }
        EntryKind::Sound { .. } => layout::add_sfx_button(ui, gdsfx, entry),
    }
}
