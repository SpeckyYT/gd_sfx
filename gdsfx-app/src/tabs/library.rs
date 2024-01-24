use std::time::Duration;

use eframe::egui::{Ui, CollapsingHeader};
use gdsfx_library::{Library, LibraryEntry, EntryKind, EntryId};

use crate::{layout, backend::{AppState, settings::SearchFilterMode}};

const UNLISTED_ID: EntryId = EntryId::MAX;

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_library_page_selection(ui, app_state);
    layout::add_search_area(ui, &mut app_state.search_settings);

    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let categories: Vec<&LibraryEntry> = library.iter_children(library.get_root()).collect();
    render_recursive(ui, app_state, library, categories, collapse_all);

    let mut unlisted_sounds: Vec<LibraryEntry> = app_state.unlisted_sfx.iter()
        .map(|&id| LibraryEntry {
            id,
            name: id.to_string(),
            parent_id: UNLISTED_ID,
            kind: EntryKind::Sound {
                bytes: 0,
                duration: Duration::ZERO,
            },
        })
        .collect();

    let unlisted_sfx_empty = unlisted_sounds.is_empty();

    if unlisted_sfx_empty && app_state.settings.search_filter_mode == SearchFilterMode::Hide {
        return
    }

    unlisted_sounds.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));

    ui.separator();

    ui.add_enabled_ui(!unlisted_sfx_empty, |ui| {
        let collapsing = CollapsingHeader::new(t!("library.unlisted_sfx"))
            .open((unlisted_sfx_empty || collapse_all).then_some(false));

        let response = collapsing.show(ui, |ui| {
            for entry in unlisted_sounds {
                layout::add_sfx_button(ui, app_state, library, &entry);
            }
        });

        let text = t!("library.unlisted_sfx.hint", tool = t!("tools.download_from_range"));
        response.header_response.on_disabled_hover_text(text);
    });
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library: &Library, mut entries: Vec<&LibraryEntry>, collapse_all: bool) {
    entries.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));
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
