use eframe::egui::{Ui, ComboBox};

use crate::GdSfx;

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);

    ui.checkbox(&mut gdsfx.settings.hide_empty_categories, t!("settings.hide_empty_categories"));

    ComboBox::from_label(t!("settings.language"))
        .selected_text(t!("language.name"))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                ui.selectable_value(&mut gdsfx.settings.locale, locale.to_string(), t!("language.name", locale = locale));
            }
        });

    gdsfx.settings.try_save_if_changed().unwrap(); // TODO error modal?

    rust_i18n::set_locale(&gdsfx.settings.locale);
}
