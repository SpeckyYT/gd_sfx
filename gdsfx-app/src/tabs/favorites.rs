use eframe::egui::Ui;
use gdsfx_library::{MusicLibrary, SfxLibrary};
use gdsfx_library::sfx::SfxLibraryEntry;

use crate::{layout, backend::{AppState, LibraryPage}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);
    layout::add_search_area(ui, &mut app_state.search_settings);

    match app_state.library_page {
        LibraryPage::Sfx => {
            let mut sounds: Vec<&SfxLibraryEntry> = sfx_library.iter_sounds().collect();
            sounds.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));
        
            for sound in sounds {
                if app_state.favorites.has_favorite(sound.id) {
                    layout::add_sfx_button(ui, app_state, sfx_library, sound);
                }
            }
        },
        LibraryPage::Music => {
            for song in &music_library.songs {
                if app_state.favorites.has_favorite(song.id) {
                    layout::add_music_button(ui, app_state, song);
                }
            }
        },
    }
}
