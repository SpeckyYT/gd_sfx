use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{GdSfx, layout, settings::SearchFilterMode};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    layout::add_search_area(ui, gdsfx);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let root = gdsfx.library.get_root().clone();
    if let EntryKind::Category { children } = root.kind {
        for child in children {
            render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone(), collapse_all)
        }
    }

    // TODO do unlisted fuckery
}

fn render_recursive(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry, collapse: bool) {
    match entry.kind {
        EntryKind::Category { ref children } => {
            let is_enabled = gdsfx.is_matching_entry(entry.clone());
            
            if !is_enabled && gdsfx.settings.search_filter_mode == SearchFilterMode::Hide {
                return // don't render at all
            }

            ui.add_enabled_ui(is_enabled, |ui| {
                let mut collapsing = CollapsingHeader::new(&entry.name);

                if !is_enabled || collapse {
                    collapsing = collapsing.open(Some(false)); // closes it
                }
                
                collapsing.show(ui, |ui| {
                    for &child in children {
                        render_recursive(ui, gdsfx, gdsfx.library.get_entry(child).clone(), collapse);
                    }
                });
            });
        }
        EntryKind::Sound { .. } => layout::add_sfx_button(ui, gdsfx, entry),
    }
}
