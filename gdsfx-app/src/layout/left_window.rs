use eframe::egui::{Context, SidePanel, ScrollArea};
use gdsfx_library::{MusicLibrary, SfxLibrary};

use crate::{layout, tabs, backend::AppState};

pub fn render(ctx: &Context, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    SidePanel::left("left_panel")
        .min_width(layout::MIN_LIBRARY_WIDTH)
        .max_width(layout::RIGHT_PANEL_WIDTH)
        .default_width(layout::DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                use tabs::*;
                
                match app_state.selected_tab {
                    Tab::Library => library::render(ui, app_state, sfx_library, music_library),
                    Tab::Favorites => favorites::render(ui, app_state, sfx_library, music_library),
                    Tab::Tools => tools::render(ui, ctx, app_state, sfx_library),
                    Tab::Settings => settings::render(ui, app_state),
                    Tab::Stats => stats::render(ui, app_state, sfx_library),
                    Tab::Credits => credits::render(ui, app_state, sfx_library, music_library),
                }
            });
        });
}
