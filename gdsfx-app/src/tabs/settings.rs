use eframe::egui::{Ui, ComboBox};
use strum::IntoEnumIterator;

use crate::app_state::{AppState, settings::*};

pub fn render(ui: &mut Ui, app_state: &mut AppState) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);

    ComboBox::from_label(t!("settings.search_filter_mode"))
        .selected_text(format!("{:?}", app_state.settings.search_filter_mode)) // TODO
        .show_ui(ui, |ui| {
            for mode in SearchFilterMode::iter() {
                ui.selectable_value(&mut app_state.settings.search_filter_mode, mode, format!("{mode:?}")); // TODO
            }
        });
    
    ui.add_space(10.0);

    ComboBox::from_label(t!("settings.sfx_select_mode"))
        .selected_text(format!("{:?}", app_state.settings.sfx_select_mode)) // TODO
        .show_ui(ui, |ui| {
            for mode in SfxSelectMode::iter() {
                ui.selectable_value(&mut app_state.settings.sfx_select_mode, mode, format!("{mode:?}")); // TODO
            }
        });

    ui.checkbox(&mut app_state.settings.play_sfx_on_click, t!("settings.play_sfx_on_click"));
    
    ui.add_space(10.0);

    ComboBox::from_label(t!("settings.language"))
        .selected_text(t!("language.name"))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                ui.selectable_value(&mut app_state.settings.locale, locale.to_string(), t!("language.name", locale = locale));
            }
        });

        app_state.settings.try_save_if_changed().unwrap(); // TODO error modal? or just dont unwrap

    rust_i18n::set_locale(&app_state.settings.locale);
}
