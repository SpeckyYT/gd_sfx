use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{Library, LibraryEntry, EntryKind};

use crate::{layout, backend::{AppState, settings::SearchFilterMode}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let categories = library.iter_children(library.get_root()).collect();
    render_recursive(ui, app_state, library, categories, collapse_all);

    // TODO do unlisted fuckery
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library: &Library, mut entries: Vec<&LibraryEntry>, collapse_all: bool) {
    app_state.search_settings.sorting_mode.sort_entries(&mut entries);
    for entry in entries {
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
                        render_recursive(ui, app_state, library, library.iter_children(entry).collect(), collapse_all);
                    });
                });
            }
            EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library, entry),
        }
    }
}
