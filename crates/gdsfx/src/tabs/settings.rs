use eframe::{egui::*, emath::Align};
use strum::IntoEnumIterator;

use crate::{backend::{AppState, settings::PersistentSettings}, i18n::LocalizedEnum, layout};

pub fn render(ui: &mut Ui, app_state: &mut AppState) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);
    set_enum_setting(ui, &mut app_state.settings.search_filter_mode);
    
    ui.add_space(10.0);
    set_enum_setting(ui, &mut app_state.settings.sfx_select_mode);
    ui.checkbox(&mut app_state.settings.play_sfx_on_click, t!("settings.play_sfx_on_click"));
    
    ui.add_space(10.0);
    set_locale(ui, app_state);

    ui.add_space(10.0);
    set_enum_setting(ui, &mut app_state.settings.theme);

    ui.add_space(10.0);

    select_gd_folder(ui, app_state);

    reset_settings(ui, app_state);

    app_state.settings.try_save_if_changed();
}

fn set_enum_setting<T>(ui: &mut Ui, selected: &mut T)
where
    T: LocalizedEnum + IntoEnumIterator + PartialEq + Copy,
{
    ComboBox::from_label(T::localize_enum())
        .selected_text(selected.localize_variant())
        .show_ui(ui, |ui| {
            for mode in T::iter() {
                ui.selectable_value(selected, mode, mode.localize_variant());
            }
        });
}

fn set_locale(ui: &mut Ui, app_state: &mut AppState) {
    ComboBox::from_label(t!("settings.language"))
        .selected_text(t!("language.name"))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                // arabic text cannot be rendered yet sadly :(
                // https://github.com/emilk/egui/issues/3378
                if locale == "ar_EG" { continue }

                ui.selectable_value(
                    &mut app_state.settings.locale,
                    locale.to_string(), t!("language.name", locale = locale)
                );
            }
        });

    rust_i18n::set_locale(&app_state.settings.locale);
}

fn select_gd_folder(ui: &mut Ui, app_state: &mut AppState) {
    ui.label(t!("settings.gd_folder"));

    let is_invalid = !app_state.is_gd_folder_valid();
    let text_edit = TextEdit::singleline(&mut app_state.settings.gd_folder)
        .desired_width(f32::INFINITY)
        .text_color_opt(is_invalid.then_some(Color32::LIGHT_RED));

    ui.add_enabled(false, text_edit);

    let button = Button::new(t!("settings.gd_folder.select"));
    let response = ui.add_enabled(!app_state.is_tool_running(), button)
        .on_disabled_hover_text(t!("settings.cannot_modify.tool_running"));
    
    if response.clicked() {
        let file_dialog = rfd::FileDialog::new()
            .set_directory(&app_state.settings.gd_folder);

        if let Some(folder) = file_dialog.pick_folder() {
            app_state.settings.gd_folder = folder.display().to_string();
        }
    }
}

fn reset_settings(ui: &mut Ui, app_state: &mut AppState) {
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        ui.add_space(4.0);

        if layout::add_caution_button(ui, t!("settings.reset")).triple_clicked() {
            app_state.settings = PersistentSettings::default();
        }
        
        ui.label(t!("settings.reset.instruction"));
    });
}
