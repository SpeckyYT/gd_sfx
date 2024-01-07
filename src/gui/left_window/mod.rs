mod library_tab;
mod favourites_tab;
mod tools_tab;
mod settings_tab;
mod stats_tab;
mod credits_tab;

use eframe::egui::{Ui, Context, SidePanel, ScrollArea};

use crate::{library::LibraryEntry, settings, audio};

use super::{GdSfx, Tab, Sorting};

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    SidePanel::left("left_panel").show(ctx, |ui| {
        if let Tab::Library | Tab::Favourites = gdsfx.tab {
            add_search_area(ui, gdsfx);
        }
        
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(sfx_library) = &gdsfx.sfx_library {
                let mut library = sfx_library.sound_effects.clone();
                filter_sounds(gdsfx, &mut library);
                match gdsfx.tab {
                    Tab::Library => library_tab::render(ui, gdsfx, library),
                    Tab::Favourites => favourites_tab::render(ui, gdsfx, library),
                    Tab::Tools => tools_tab::render(ui, gdsfx, ctx),
                    Tab::Settings => settings_tab::render(ui, gdsfx),
                    Tab::Stats => stats_tab::render(ui, gdsfx),
                    Tab::Credits => credits_tab::render(ui, gdsfx),
                }
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
            (Sorting::IdDec,     t!("sort.id.ascending")),  // this is not a bug, in gd, the id sorting is reversed,
            (Sorting::IdInc,     t!("sort.id.descending")), // in-game it's `ID+ => 9 - 0; ID- => 0 - 9`
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
}

fn add_sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    if !entry.is_enabled() { return }

    let sound = ui.button(entry.pretty_name());

    let entry_selected = sound.hovered();

    if sound.clicked() {
        audio::stop_audio();
        audio::play_sound(&entry);
    }

    sound.context_menu(|ui| {
        if settings::has_favourite(entry.id()) {
            if ui.button(t!("sound.button.favorite.remove")).clicked() {
                settings::remove_favourite(entry.id());
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.favorite.add")).clicked() {
            settings::add_favourite(entry.id());
            ui.close_menu();
        }

        if entry.exists() {
            if ui.button(t!("sound.button.delete")).clicked() {
                entry.delete();
                ui.close_menu();
            }
        } else if ui.button(t!("sound.button.download")).clicked() {
            entry.download_and_store();
            ui.close_menu();
        }
    });

    if entry_selected {
        gdsfx.selected_sfx = Some(entry);
    }
}
