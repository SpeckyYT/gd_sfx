use eframe::egui::{Context, SidePanel, Button};
use strum::IntoEnumIterator;

use crate::{tabs::Tab, backend::AppState};

pub fn render(ctx: &Context, app_state: &mut AppState) {
    SidePanel::left("tabs_panel")
    .default_width(64.0)
    .resizable(false)
    .show(ctx, |ui| {
        ui.add_space(4.0);
        ui.vertical_centered(|ui| {
            for tab in Tab::iter() {
                let tab_element = ui.add(Button::image(tab.icon()));

                if tab_element.clicked() {
                    app_state.selected_tab = tab;
                }
            }
        });
        ui.add_space(2.0);
    });
}
