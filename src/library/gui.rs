use eframe::egui::Ui;

use crate::{gui::{GdSfx, Sorting, self}, library::LibraryEntry, settings::SETTINGS};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    match entry {
        LibraryEntry::Category { name, parent, mut children, enabled, .. } => {
            children.sort_by(|a: &LibraryEntry, b: &LibraryEntry| {
                b.is_category().cmp(&a.is_category()) // categories on top
                    .then(match gdsfx.sorting {
                        Sorting::Default => std::cmp::Ordering::Equal,
                        Sorting::NameInc => a.name().cmp(b.name()),
                        Sorting::NameDec => b.name().cmp(a.name()),
                        Sorting::LengthInc => a.duration().cmp(&b.duration()),
                        Sorting::LengthDec => b.duration().cmp(&a.duration()),
                        Sorting::IdInc => a.id().cmp(&b.id()),
                        Sorting::IdDec => b.id().cmp(&a.id()),
                        Sorting::SizeInc => a.bytes().cmp(&b.bytes()),
                        Sorting::SizeDec => b.bytes().cmp(&a.bytes()),
                    })
            });

            if parent == 0 { // root
                for child in children {
                    render(ui, gdsfx, child);
                }
            } else if enabled || !SETTINGS.lock().hide_empty_categories {
                ui.add_enabled_ui(enabled, |ui| {
                    ui.collapsing(name, |ui| {
                        for child in children {
                            render(ui, gdsfx, child);
                        }
                    });
                });
            }
        }
        LibraryEntry::Sound { .. } => gui::add_sfx_button(ui, gdsfx, entry)
    }
}