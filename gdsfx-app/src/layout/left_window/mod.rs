use eframe::egui::{Ui, Context, SidePanel, ScrollArea};
use gdsfx_library::{get_library, LibraryEntry};

mod library;
mod favorites;
mod tools;
mod settings;
mod stats;
mod credits;

use crate::{GdSfx, Tab, Sorting};

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
            let mut library = get_library().library().clone();
            // filter_sounds(gdsfx, &mut library);
            match gdsfx.selected_tab {
                Tab::Library => library::render(ui, gdsfx, library),
                Tab::Favourites => favorites::render(ui, gdsfx, library),
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

fn filter_sounds(gdsfx: &mut GdSfx, node: &mut LibraryEntry) {
    /*
    match node {
        LibraryEntry::Sound { .. } => {
            node.set_enabled(gdsfx.matches_query(node));
        }
        LibraryEntry::Category { children, .. } => {
            for child in children.iter_mut() {
                filter_sounds(gdsfx, child);
            }

            let any_enabled = children.iter().any(LibraryEntry::is_enabled);
            node.set_enabled(any_enabled);
        }
    }
    */
}
