use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{layout, library_manager::LibraryManager, app_state::{AppState, settings::SearchFilterMode}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let root = library_manager.library.get_root();
    for child in library_manager.library.get_children(root) {
        render_recursive(ui, app_state, library_manager, child, collapse_all);
    }

    // TODO do unlisted fuckery
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager, entry: &LibraryEntry, collapse_all: bool) {
    match entry.kind {
        EntryKind::Category => {
            let is_enabled = library_manager.is_matching_entry(entry, &app_state.search_settings);

            if !is_enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide {
                return // don't render at all
            }

            ui.add_enabled_ui(is_enabled, |ui| {
                let mut collapsing = CollapsingHeader::new(&entry.name);

                if !is_enabled || collapse_all {
                    collapsing = collapsing.open(Some(false));
                }

                collapsing.show(ui, |ui| {
                    for child in library_manager.library.get_children(entry) {
                        render_recursive(ui, app_state, library_manager, child, collapse_all);
                    }
                });
            });
        }
        EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library_manager, entry),
    }
}
