use eframe::egui::{Context, TopBottomPanel};
use strum::IntoEnumIterator;

use crate::{GdSfx, Tab};

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for tab in Tab::iter() {
                ui.selectable_value(
                    &mut gdsfx.selected_tab,
                    tab,
                    tab.get_localized_name(),
                );
            }
        });
        ui.add_space(2.0);
    });
}
