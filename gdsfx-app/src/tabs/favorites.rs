use eframe::egui::Ui;
use gdsfx_library::{Library, LibraryEntry};

use crate::{layout, backend::AppState};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let mut sounds: Vec<&LibraryEntry> = library.iter_sounds().collect();
    sounds.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));

    for sound in sounds {
        if app_state.favorites.has_favorite(sound.id) {
            layout::add_sfx_button(ui, app_state, library, sound);
        }
    }
}
