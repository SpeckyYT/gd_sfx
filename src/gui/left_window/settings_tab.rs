use eframe::egui::{Ui, ComboBox};

use crate::{gui::GdSfx, util, settings::{SETTINGS, self}};

pub fn render(ui: &mut Ui, _gdsfx: &mut GdSfx) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);

    let mut settings = SETTINGS.lock().unwrap();
    let initial_settings = *settings;

    ui.checkbox(&mut settings.hide_empty_categories, t!("settings.hide_empty_categories"));

    let mut current_locale = rust_i18n::locale();
    let initial_locale = current_locale.clone();

    ComboBox::from_label(t!("settings.language"))
        .selected_text(util::format_locale(&current_locale))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                ui.selectable_value(&mut current_locale, locale.to_string(), util::format_locale(locale));
            }
        });

    if *settings != initial_settings || current_locale != initial_locale {
        drop(settings); // fixes deadlock (geometry dash reference)
        rust_i18n::set_locale(&current_locale);
        settings::save();
    }
}