use std::time::Duration;

use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{Library, LibraryEntry, EntryKind, EntryId};

use crate::{layout, backend::{AppState, settings::SearchFilterMode}};

const UNLISTED_ID: EntryId = EntryId::MAX;

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let categories: Vec<&LibraryEntry> = library.iter_children(library.get_root()).collect();
    render_recursive(ui, app_state, library, categories, collapse_all);
    
    ui.separator();

    ui.add_enabled_ui(!app_state.unlisted_sounds.is_empty(), |ui| {
        let collapsing = CollapsingHeader::new(t!("library.unlisted_sfx")).open(collapse_all.then_some(false));

        let response = collapsing.show(ui, |ui| {
            let mut unlisted_sounds: Vec<LibraryEntry> = app_state.unlisted_sounds.iter()
                .map(|id|
                    LibraryEntry {
                        id: *id,
                        name: id.to_string(),
                        parent_id: UNLISTED_ID,
                        kind: EntryKind::Sound { bytes: 0, duration: Duration::ZERO },
                    }
                )
                .filter(|entry| app_state.is_matching_entry(entry, library))
                .collect();

            unlisted_sounds.sort_by(app_state.search_settings.sorting_mode.comparator());

            for entry in unlisted_sounds {
                layout::add_sfx_button(ui, app_state, library, &entry);
            }
        });

        response.header_response.on_disabled_hover_text(" if u no unlisted then use download from id range tool :)_)");
    });
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library: &Library, mut entries: Vec<&LibraryEntry>, collapse_all: bool) {
    entries.sort_by(app_state.search_settings.sorting_mode.comparator());
    for entry in entries {
        match entry.kind {
            EntryKind::Category => {
                let is_enabled = app_state.is_matching_entry(entry, library);
    
                if !is_enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide {
                    continue // skip rendering entirely
                }
    
                ui.add_enabled_ui(is_enabled, |ui| {
                    let collapsing = CollapsingHeader::new(&entry.name)
                        .open((!is_enabled || collapse_all).then_some(false));
    
                    collapsing.show(ui, |ui| {
                        render_recursive(ui, app_state, library, library.iter_children(entry).collect(), collapse_all);
                    });
                });
            }
            EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library, entry),
        }
    }
}
