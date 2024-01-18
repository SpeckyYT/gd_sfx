use eframe::egui::Ui;
use gdsfx_library::Library;

use crate::{layout, backend::AppState};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library: &Library) {
    layout::add_search_area(ui, &mut app_state.search_settings);

    let mut sounds = library.iter_sounds().collect::<Vec<_>>();
    sounds.sort_by(app_state.search_settings.sorting_mode.comparator());

    for sound in sounds {
        if app_state.favorites.has_favorite(sound.id) {
            layout::add_sfx_button(ui, app_state, library, sound);
        }
    }
}
