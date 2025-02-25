use eframe::egui::{ScrollArea, Ui};
use itertools::Itertools;

use gdsfx_library::{MusicLibrary, SfxLibrary};

use crate::{layout, backend::{AppState, LibraryPage}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    ui.heading(t!("tab.favorites"));

    layout::add_search_area(ui, &mut app_state.search_settings);

    ui.separator();

    ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        match app_state.library_page {
            LibraryPage::Sfx => app_state.favorites.iter()
                .flat_map(|id| sfx_library.entries.get(&id))
                .sorted_by(|&a, &b| app_state.search_settings.sorting_mode.compare_entries(a, b))
                .for_each(|sound| layout::add_sfx_button(ui, app_state, sfx_library, sound)),

            LibraryPage::Music => app_state.favorites.iter()
                .flat_map(|id| music_library.songs.get(&id))
                .sorted_by(|&a, &b| app_state.search_settings.sorting_mode.compare_entries(a, b))
                .for_each(|sound| layout::add_music_button(ui, app_state, sound)),
        }
    });
}
