use eframe::{egui::{Button, Context, SidePanel}, epaint::Vec2};
use strum::IntoEnumIterator;

use crate::{tabs::Tab, backend::AppState};
use crate::i18n::LocalizedEnum;

const TAB_SIZE: Vec2 = Vec2 { x: 48.0, y: 48.0 };

pub fn render(ctx: &Context, app_state: &mut AppState) {
    SidePanel::left("tabs_panel")
    .default_width(TAB_SIZE.x)
    .resizable(false)
    .show(ctx, |ui| {
        ui.add_space(5.0);
        ui.vertical_centered(|ui| {
            for tab in Tab::iter() {
                let icon = tab.icon().max_size(TAB_SIZE);
                let tab_element = ui.add(Button::image(icon).min_size(TAB_SIZE))
                    .on_hover_text(tab.localize_variant());

                if tab_element.clicked() {
                    app_state.selected_tab = tab;
                }
            }
        });
    });
}
