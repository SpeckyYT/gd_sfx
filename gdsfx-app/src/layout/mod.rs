use eframe::{egui::Ui, epaint::Vec2};
use gdsfx_library::LibraryEntry;

use crate::{settings::SfxSelectMode, sorting::Sorting, app_state::AppState, library_manager::LibraryManager};

pub mod top_panel;
pub mod left_window;
pub mod right_window;

pub const MIN_LIBRARY_WIDTH: f32 = 200.0;
pub const DEFAULT_LIBRARY_WIDTH: f32 = 300.0;
pub const RIGHT_PANEL_WIDTH: f32 = 500.0;

pub const TOTAL_WIDTH: f32 = DEFAULT_LIBRARY_WIDTH + RIGHT_PANEL_WIDTH;
pub const TOTAL_HEIGHT: f32 = 600.0; // enough to display all categories

pub const DEFAULT_WINDOW_SIZE: Vec2 = Vec2 { x: TOTAL_WIDTH, y: TOTAL_HEIGHT };
pub const MIN_SCALE_FACTOR: f32 = 0.7;

pub fn add_search_area(ui: &mut Ui, app_state: &mut AppState) {
    ui.heading(t!("search"));
    ui.text_edit_singleline(&mut app_state.search_query);

    ui.menu_button(t!("sort.button"), |ui| {
        for (alternative, text) in [
            (Sorting::Default,   t!("sort.default")),
            (Sorting::NameInc,   t!("sort.name.ascending")),
            (Sorting::NameDec,   t!("sort.name.descending")),
            (Sorting::LengthInc, t!("sort.length.ascending")),
            (Sorting::LengthDec, t!("sort.length.descending")),
            (Sorting::IdInc,     t!("sort.id.ascending")),
            (Sorting::IdDec,     t!("sort.id.descending")),
            (Sorting::SizeInc,   t!("sort.size.ascending")),
            (Sorting::SizeDec,   t!("sort.size.descending")),
        ] {
            let response = ui.radio_value(&mut app_state.sorting_mode, alternative, text);
            if response.clicked() {
                ui.close_menu();
            }
        }
    });

    ui.separator();
}

pub fn add_sfx_button(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager, entry: LibraryEntry) {
    // don't render filtered buttons at all
    if !library_manager.is_matching_entry(&entry, &app_state.search_query) { return }

    let button = ui.button(match app_state.settings.is_favorite(entry.id) {
        true => format!("⭐ {}", entry.name),
        false => entry.name.to_string(),
    });

    let entry_selected = match app_state.settings.sfx_select_mode {
        SfxSelectMode::Hover => button.hovered(),
        SfxSelectMode::Click => button.clicked(),
    };

    if button.clicked() && app_state.settings.play_sfx_on_click {
        library_manager.play_sound(&entry, app_state.audio_settings);
    }

    button.context_menu(|ui: &mut Ui| {
        if app_state.settings.is_favorite(entry.id) {
            if ui.button(t!("sound.button.favorite.remove")).clicked() {
                app_state.settings.remove_favorite(entry.id);
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.favorite.add")).clicked() {
            app_state.settings.add_favorite(entry.id);
            ui.close_menu();
        }

        if entry.file_exists() {
            if ui.button(t!("sound.button.delete")).clicked() {
                entry.try_delete_file();
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.download")).clicked() {
            library_manager.download_sound(&entry);
            ui.close_menu();
        }
    });

    if entry_selected {
        app_state.selected_sfx = Some(entry);
    }
}
