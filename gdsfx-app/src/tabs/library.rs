use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{LibraryEntry, EntryKind, Library};

use crate::{layout, backend::{AppState, settings::SearchFilterMode}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let root = library.get_root();
    let mut children = library.get_children(root).collect::<Vec<_>>();
    app_state.search_settings.sorting_mode.sort_entries(&mut children);

    for child in children {
        render_recursive(ui, app_state, library, child, collapse_all);
    }

    // TODO do unlisted fuckery
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library: &Library, entry: &LibraryEntry, collapse_all: bool) {
    match entry.kind {
        EntryKind::Category => {
            let is_enabled = app_state.is_matching_entry(entry, library);

            if !is_enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide {
                return // don't render at all
            }

            ui.add_enabled_ui(is_enabled, |ui| {
                let mut collapsing = CollapsingHeader::new(&entry.name);

                if !is_enabled || collapse_all {
                    collapsing = collapsing.open(Some(false));
                }

                collapsing.show(ui, |ui| {
                    let mut children = library.get_children(entry).collect::<Vec<_>>();
                    app_state.search_settings.sorting_mode.sort_entries(&mut children);

                    for child in children {
                        render_recursive(ui, app_state, library, child, collapse_all);
                    }
                });
            });
        }
        EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library, entry),
    }
}
