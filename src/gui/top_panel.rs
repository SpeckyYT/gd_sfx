use eframe::egui::{Context, TopBottomPanel};
use strum::IntoEnumIterator;

use super::{GdSfx, Tab};

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            Tab::iter().for_each(|tab| {
                ui.selectable_value(&mut gdsfx.tab, tab, tab.get_localized_name());
            });
        });
        ui.add_space(2.0);
    });
}
