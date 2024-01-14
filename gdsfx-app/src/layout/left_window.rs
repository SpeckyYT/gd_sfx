use eframe::egui::{Context, SidePanel, ScrollArea};

use crate::{GdSfx, layout, tabs};

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    SidePanel::left("left_panel")
        .min_width(layout::MIN_LIBRARY_WIDTH)
        .max_width(layout::RIGHT_PANEL_WIDTH)
        .default_width(layout::DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                use tabs::*;
                
                match gdsfx.selected_tab {
                    Tab::Library => library::render(ui, gdsfx),
                    Tab::Favourites => favorites::render(ui, gdsfx),
                    Tab::Tools => tools::render(ui, gdsfx, ctx),
                    Tab::Settings => settings::render(ui, gdsfx),
                    Tab::Stats => stats::render(ui, gdsfx),
                    Tab::Credits => credits::render(ui, gdsfx),
                }
            });
        });
}
