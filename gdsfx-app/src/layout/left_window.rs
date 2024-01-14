use eframe::egui::{Context, SidePanel, ScrollArea};

use crate::{GdSfx, layout, tabs};

pub fn render(ctx: &Context, gdsfx: &mut GdSfx) {
    SidePanel::left("left_panel")
        .min_width(layout::MIN_LIBRARY_WIDTH)
        .max_width(layout::RIGHT_PANEL_WIDTH)
        .default_width(layout::DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                use tabs::*;

                let app_state = &mut gdsfx.app_state;
                let library_manager = &gdsfx.library_manager;
                
                match app_state.selected_tab {
                    Tab::Library => library::render(ui, app_state, library_manager),
                    Tab::Favourites => favorites::render(ui, app_state, library_manager),
                    Tab::Tools => tools::render(ui, ctx),
                    Tab::Settings => settings::render(ui, app_state),
                    Tab::Stats => stats::render(ui, library_manager),
                    Tab::Credits => credits::render(ui, library_manager),
                }
            });
        });
}
