use eframe::{egui::Ui, epaint::Vec2};
use gdsfx_library::LibraryEntry;

use crate::{app_state::{AppState, settings::*, search::{SearchSettings, Sorting}}, library_manager::LibraryManager};

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

pub fn add_search_area(ui: &mut Ui, search_settings: &mut SearchSettings) {
    ui.heading(t!("search"));
    ui.text_edit_singleline(&mut search_settings.search_query);
    
    ui.horizontal(|ui| {
        ui.menu_button(t!("search.sorting"), |ui| {
            for (alternative, text) in [
                (Sorting::Default,   t!("sorting.default")),
                (Sorting::NameInc,   t!("sorting.name.ascending")),
                (Sorting::NameDec,   t!("sorting.name.descending")),
                (Sorting::LengthInc, t!("sorting.length.ascending")),
                (Sorting::LengthDec, t!("sorting.length.descending")),
                (Sorting::IdInc,     t!("sorting.id.ascending")),
                (Sorting::IdDec,     t!("sorting.id.descending")),
                (Sorting::SizeInc,   t!("sorting.size.ascending")),
                (Sorting::SizeDec,   t!("sorting.size.descending")),
            ] {
                let response = ui.radio_value(&mut search_settings.sorting_mode, alternative, text);
                if response.clicked() {
                    ui.close_menu();
                }
            }
        });

        ui.checkbox(&mut search_settings.filter_downloaded, t!("search.filter_downloaded"));
    });

    ui.separator();
}

pub fn add_sfx_button(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager, entry: &LibraryEntry) {
    if !library_manager.is_matching_entry(entry, &app_state.search_settings) {
        return // don't render filtered buttons at all
    }

    let button = ui.button(match app_state.favorites.has_favorite(entry.id) {
        true => format!("⭐ {}", entry.name),
        false => entry.name.to_string(),
    });

    if match app_state.settings.sfx_select_mode {
        SfxSelectMode::Hover => button.hovered(),
        SfxSelectMode::Click => button.clicked(),
    } {
        app_state.selected_sfx = Some(entry.clone());
    }

    if button.clicked() && app_state.settings.play_sfx_on_click {
        library_manager.play_sound(entry, app_state);
    }

    button.context_menu(|ui: &mut Ui| {
        if app_state.favorites.has_favorite(entry.id) {
            if ui.button(t!("sound.button.favorite.remove")).clicked() {
                app_state.favorites.remove_favorite(entry.id);
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.favorite.add")).clicked() {
            app_state.favorites.add_favorite(entry.id);
            ui.close_menu();
        }

        if entry.file_exists(app_state.settings.gd_folder.as_ref()) {
            if ui.button(t!("sound.button.delete")).clicked() {
                entry.try_delete_file(app_state.settings.gd_folder.as_ref());
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.download")).clicked() {
            library_manager.download_sound(entry, app_state);
            ui.close_menu();
        }
    });
}
