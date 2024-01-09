use eframe::egui::Ui;

use crate::{gui::{GdSfx, Sorting, self}, library::LibraryEntry, settings::SETTINGS, util::UNLISTED_SFX};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, entry: LibraryEntry) {
    match entry {
        LibraryEntry::Category { id, name, parent, mut children, enabled, .. } => {
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

            if id == 1 {
                children.push(LibraryEntry::Category {
                    id: u32::MAX,
                    name: "Unlisted SFX".to_string(),
                    parent: 1,
                    children: {
                        let unlisted_sfx = UNLISTED_SFX.lock();
                        let mut sfxes: Vec<_> = unlisted_sfx.iter().copied().collect();
                        drop(unlisted_sfx);
                        sfxes.sort_unstable();
                        sfxes.into_iter()
                        .enumerate()
                        .map(|(i, id)| {
                            LibraryEntry::Sound {
                                id,
                                name: format!("Unused SFX #{}", i + 1), // lua simulator
                                parent: u32::MAX,
                                bytes: 0,
                                duration: 0,
                                enabled: true,
                            }
                        })
                        .collect()
                    },
                    enabled: true,
                });
            }

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