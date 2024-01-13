use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, sorting::Sorting};

use crate::GdSfx;

pub mod top_panel;
pub mod left_window;
pub mod right_window;

pub fn add_search_area(ui: &mut Ui, gdsfx: &mut GdSfx) {
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

pub fn add_sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    // TODO gray out/hide behavior

    let sound = ui.button(&entry.name); // TODO with favorites star

    let entry_selected = sound.hovered();

    if sound.clicked() {
        gdsfx_audio::stop_all();
        // gdsfx_audio::play_sound(&entry);
    }

    sound.context_menu(|ui| {
        // if settings::has_favourite(entry.id()) {
        //     if ui.button(t!("sound.button.favorite.remove")).clicked() {
        //         settings::remove_favourite(entry.id());
        //         ui.close_menu();
        //     }
        // } else if ui.button(t!("sound.button.favorite.add")).clicked() {
        //     settings::add_favourite(entry.id());
        //     ui.close_menu();
        // }

        // if entry.exists() {
        //     if ui.button(t!("sound.button.delete")).clicked() {
        //         entry.delete();
        //         ui.close_menu();
        //     }
        // } else if ui.button(t!("sound.button.download")).clicked() {
        //     entry.download_and_store();
        //     ui.close_menu();
        // }
    });

    if entry_selected {
        gdsfx.selected_sfx = Some(entry);
    }
}
