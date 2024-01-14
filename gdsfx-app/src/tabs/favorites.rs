use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{layout, app_state::AppState, library_manager::LibraryManager};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let mut sounds = get_sounds_recursive(library_manager, library_manager.library.get_root());
    app_state.search_settings.sorting_mode.sort_entries(&mut sounds);

    for sound in sounds {
        if app_state.favorites.has_favorite(sound.id) {
            layout::add_sfx_button(ui, app_state, library_manager, sound);
        }
    }
}

fn get_sounds_recursive<'a>(library_manager: &'a LibraryManager, entry: &'a LibraryEntry) -> Vec<&'a LibraryEntry> {
    match &entry.kind {
        EntryKind::Category => {
            library_manager.library.get_children(entry)
                .flat_map(|entry| get_sounds_recursive(library_manager, entry))
                .collect()
        }
        EntryKind::Sound { .. } => vec![entry],
    }
}
