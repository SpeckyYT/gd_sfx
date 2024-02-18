use eframe::{egui::*, epaint::Vec2};
use egui_modal::ModalStyle;
use gdsfx_library::{music::Song, SfxLibrary};
use gdsfx_library::sfx::SfxLibraryEntry;
use strum::IntoEnumIterator;

use crate::{backend::{AppState, settings::*, search::*, LibraryPage}, i18n::LocalizedEnum, images};

pub mod tabs_panel;
pub mod left_window;
pub mod right_window;
pub mod debug_window;

pub const MIN_LIBRARY_WIDTH: f32 = 200.0;
pub const DEFAULT_LIBRARY_WIDTH: f32 = 300.0;
pub const RIGHT_PANEL_WIDTH: f32 = 500.0;

pub const TOTAL_WIDTH: f32 = DEFAULT_LIBRARY_WIDTH + RIGHT_PANEL_WIDTH;
pub const TOTAL_HEIGHT: f32 = 650.0; // enough to display all categories

pub const DEFAULT_WINDOW_SIZE: Vec2 = Vec2 { x: TOTAL_WIDTH, y: TOTAL_HEIGHT };
pub const MIN_SCALE_FACTOR: f32 = 0.7;

pub const FAVORITE_ALPHA: u8 = 100;

pub fn add_library_page_selection(ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        for page in LibraryPage::iter() {
            ui.selectable_value(&mut app_state.library_page, page, page.localize_variant());
        }
    });
    ui.separator();
}

pub fn add_search_area(ui: &mut Ui, search_settings: &mut SearchSettings) {
    ui.heading(t!("search"));

    ui.add(TextEdit::singleline(&mut search_settings.search_query).hint_text(t!("search")));
    
    ui.horizontal(|ui| {
        let label = format!("{}: {}", SortingMode::localize_enum(), search_settings.sorting_mode.localize_variant());
        ui.menu_button(label, |ui| {
            for mode in SortingMode::iter() {
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

pub fn add_sfx_button(ui: &mut Ui, app_state: &mut AppState, library: &SfxLibrary, entry: &SfxLibraryEntry) {
    if !app_state.is_matching_entry(entry, library) {
        return // don't render filtered buttons at all
    }

    let image = app_state.favorites
        .has_favorite(entry.id)
        .then_some(Image::new(images::FAVORITE_STAR).tint(Color32::from_white_alpha(FAVORITE_ALPHA))); // set opacity 0-255

    let text = WidgetText::from(&entry.name);
    let button = ui.add(Button::opt_image_and_text(image, Some(text)));

    if match app_state.settings.sfx_select_mode {
        SelectMode::Hover => button.hovered(),
        SelectMode::Click => button.clicked(),
    } {
        app_state.selected_sfx = Some(entry.clone());
    }

    if button.clicked() && app_state.settings.play_sfx_on_click {
        app_state.play_sound(entry.into_file_entry());
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

        if app_state.is_gd_folder_valid() {
            if app_state.is_sfx_downloaded(entry.id) {
                if ui.button(t!("sound.delete")).clicked() {
                    app_state.delete_sound(entry.into_file_entry());
                    ui.close_menu();
                }
            } else if ui.button(t!("sound.download")).clicked() {
                app_state.download_sound(entry.into_file_entry());
                ui.close_menu();
            }
        }
    });
}

pub fn add_music_button(ui: &mut Ui, app_state: &mut AppState, song: &Song) {
    if !app_state.is_matching_song(song) {
        return
    }

    let image = app_state.favorites
    .has_favorite(song.id)
    .then_some(Image::new(images::FAVORITE_STAR).tint(Color32::from_white_alpha(FAVORITE_ALPHA))); // set opacity 0-255

    let text = WidgetText::from(&song.name);
    let button = ui.add(Button::opt_image_and_text(image, Some(text)));

    if match app_state.settings.sfx_select_mode {
        SelectMode::Hover => button.hovered(),
        SelectMode::Click => button.clicked(),
    } {
        app_state.selected_music = Some(song.clone());
    }

    if button.clicked() && app_state.settings.play_sfx_on_click {
        app_state.play_sound(song.into_file_entry());
    }

    button.context_menu(|ui: &mut Ui| {
        let favorite_button_label = match app_state.favorites.has_favorite(song.id) {
            false => t!("sound.favorite.add"),
            true => t!("sound.favorite.remove"),
        };
        if ui.button(favorite_button_label).clicked() {
            app_state.favorites.toggle_favorite(song.id);
            ui.close_menu();
        }

        if app_state.is_gd_folder_valid() {
            if app_state.is_music_downloaded(song.id) {
                if ui.button(t!("sound.delete")).clicked() {
                    app_state.delete_sound(song.into_file_entry());
                    ui.close_menu();
                }
            } else if ui.button(t!("sound.download")).clicked() {
                app_state.download_sound(song.into_file_entry());
                ui.close_menu();
            }
        }
    });
}

pub fn add_caution_button(ui: &mut Ui, text: impl Into<WidgetText>) -> Response {
    let default_modal_style = ModalStyle::default();
    let widget_text = text.into().color(default_modal_style.caution_button_text_color);
    let button = Button::new(widget_text).fill(default_modal_style.caution_button_fill);
    ui.add(button)
}
