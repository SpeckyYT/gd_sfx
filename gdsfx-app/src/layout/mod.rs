use eframe::{egui::{self, *}, epaint::Vec2};
use gdsfx_library::{LibraryEntry, Library};
use strum::IntoEnumIterator;

use crate::{backend::{AppState, settings::*, search::*}, i18n::LocalizedEnum};

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

    ui.add(TextEdit::singleline(&mut search_settings.search_query).hint_text(t!("search")));
    
    ui.horizontal(|ui| {
        let label = format!("{}: {}", Sorting::localize_enum(), search_settings.sorting_mode.localize_variant());
        ui.menu_button(label, |ui| {
            for mode in Sorting::iter() {
                let radio_button = ui.radio_value(
                    &mut search_settings.sorting_mode,
                    mode, mode.localize_variant()
                );
                if radio_button.clicked() {
                    ui.close_menu();
                }
            }
        });

        ui.checkbox(&mut search_settings.show_downloaded, t!("search.show_downloaded"));
    });

    ui.separator();
}

pub fn add_sfx_button(ui: &mut Ui, app_state: &mut AppState, library: &Library, entry: &LibraryEntry) {
    const FAVORITE_ICON: ImageSource = egui::include_image!("../../../assets/twemoji-white-medium-star.png");

    if !app_state.is_matching_entry(entry, library) {
        return // don't render filtered buttons at all
    }

    let image = app_state.favorites
        .has_favorite(entry.id)
        .then_some(Image::new(FAVORITE_ICON).tint(Color32::from_white_alpha(100))); // set opacity 0-255

    let text = WidgetText::from(&entry.name);
    let button = ui.add(Button::opt_image_and_text(image, Some(text)));

    if match app_state.settings.sfx_select_mode {
        SfxSelectMode::Hover => button.hovered(),
        SfxSelectMode::Click => button.clicked(),
    } {
        app_state.selected_sfx = Some(entry.clone());
    }

    if button.clicked() && app_state.settings.play_sfx_on_click {
        app_state.play_sound(entry);
    }

    button.context_menu(|ui: &mut Ui| {
        let favorite_button_label = match app_state.favorites.has_favorite(entry.id) {
            false => t!("sound.favorite.add"),
            true => t!("sound.favorite.remove"),
        };
        if ui.button(favorite_button_label).clicked() {
            app_state.favorites.toggle_favorite(entry.id);
            ui.close_menu();
        }

        if let Some(file_handler) = entry.create_file_handler(&app_state.settings.gd_folder) {
            if file_handler.file_exists() {
                if ui.button(t!("sound.delete")).clicked() {
                    file_handler.try_delete_file();
                    ui.close_menu();
                }
            } else if ui.button(t!("sound.download")).clicked() {
                app_state.download_sound(entry);
                ui.close_menu();
            }
        }
    });
}
