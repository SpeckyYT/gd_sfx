use eframe::egui::{Context, SidePanel, ScrollArea};

use crate::{layout, tabs, backend::AppState, Library};

pub fn render(ctx: &Context, app_state: &mut AppState, library: Library) {
    SidePanel::left("left_panel")
        .min_width(layout::MIN_LIBRARY_WIDTH)
        .max_width(layout::RIGHT_PANEL_WIDTH)
        .default_width(layout::DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                use tabs::*;
                
                match app_state.selected_tab {
                    Tab::Library => library::render(ui, app_state, library),
                    Tab::Favorites => favorites::render(ui, app_state, library),
                    Tab::Tools => tools::render(ui, ctx, app_state, library),
                    Tab::Settings => settings::render(ui, app_state),
                    Tab::Stats => stats::render(ui, library),
                    Tab::Credits => credits::render(ui, library),
                }
            });
        });
}
