use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{layout, backend::AppState, Library};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let mut sounds = get_sounds_recursive(library.clone(), library.lock().get_root());
    app_state.search_settings.sorting_mode.sort_entries(&mut sounds);

    for sound in sounds {
        if app_state.favorites.has_favorite(sound.id) {
            layout::add_sfx_button(ui, app_state, library, sound);
        }
    }
}

fn get_sounds_recursive<'a>(library: Library, entry: &'a LibraryEntry) -> Vec<&'a LibraryEntry> {
    match &entry.kind {
        EntryKind::Category => {
            library.lock()
                .get_children(entry)
                .flat_map(|entry| get_sounds_recursive(library, entry))
                .collect()
        }
        EntryKind::Sound { .. } => vec![entry],
    }
}
