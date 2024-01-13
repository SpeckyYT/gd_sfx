use eframe::egui::{Ui, Context, SidePanel, ScrollArea};

use crate::{GdSfx, Tab, Sorting, tabs};

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    SidePanel::left("left_panel")
        // .min_width(MIN_LIBRARY_WIDTH)
        // .max_width(RIGHT_PANEL_WIDTH)
        // .default_width(DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            if let Tab::Library | Tab::Favourites = gdsfx.selected_tab {
                add_search_area(ui, gdsfx);
            }
            
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

fn add_search_area(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading(t!("search"));
    ui.text_edit_singleline(&mut gdsfx.search_query);

    ui.menu_button(t!("sort.button"), |ui| {
        for (alternative, text) in [
            (Sorting::Default,   t!("sort.default")),
            (Sorting::NameInc,   t!("sort.name.ascending")),
            (Sorting::NameDec,   t!("sort.name.descending")),
            (Sorting::LengthInc, t!("sort.length.ascending")),
            (Sorting::LengthDec, t!("sort.length.descending")),
            (Sorting::IdInc,     t!("sort.id.ascending")),
            (Sorting::IdDec,     t!("sort.id.descending")),
            (Sorting::SizeInc,   t!("sort.size.ascending")),
            (Sorting::SizeDec,   t!("sort.size.descending")),
        ] {
            let response = ui.radio_value(&mut gdsfx.sorting, alternative, text);
            if response.clicked() {
                ui.close_menu();
            }
        }
    });

    ui.separator();
}
