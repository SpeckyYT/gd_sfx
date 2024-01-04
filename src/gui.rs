use std::fs;

use eframe::{egui::{self, Button}, NativeOptions, epaint::ahash::HashMap};
use egui_modal::Modal;
use pretty_bytes::converter::convert;

use crate::library::LibraryEntry;

pub type VersionType = usize;

#[derive(Default)]
pub struct GdSfx {
    pub cdn_url: Option<String>,
    pub sfx_version: Option<VersionType>,
    pub sfx_library: Option<LibraryEntry>,

    pub selected_sfx: Option<LibraryEntry>,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if ui.button("Force-update library").clicked() {
                    self.get_sfx_library(true);
                }
                if let Some(version) = self.sfx_version {
                    ui.heading(format!("Library version: {version}"));
                }

                ui.add_space(20.0);

                if let Some(sfx_library) = self.sfx_library.as_ref() {
                    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut egui::Ui, is_root: bool) {
                        match entry {
                            LibraryEntry::Category { children, .. } => {
                                if is_root {
                                    for child in children {
                                        recursive(gdsfx, child, ui, false);
                                    }
                                } else {
                                    ui.collapsing(entry.name(), |sub_ui| {
                                        for child in children {
                                            recursive(gdsfx, child, sub_ui, false);
                                        }
                                    });
                                }
                            },
                            LibraryEntry::Sound { name, .. } => {
                                if ui.button(name).clicked() {
                                    gdsfx.selected_sfx = Some(entry.clone());
                                }
                            },
                        }
                    }

                    let sfx_library = sfx_library.clone();
                    recursive(self, &sfx_library, ui, true);
                }
            });
        });
        if let Some(sfx) = self.selected_sfx.as_ref() {
            egui::SidePanel::right("right_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading(sfx.name());
                ui.collapsing("Original code", |ui| {
                    ui.code(sfx.get_string());
                });
                ui.heading(format!("ID: {}", sfx.id()));
                ui.heading(format!("Category ID: {}", sfx.parent()));
                ui.heading(format!("Bytes: {}", convert(sfx.bytes() as f64)));
                ui.heading(format!("Duration: {}s", {
                    let mut centiseconds = format!("{:>03}", sfx.duration());
                    centiseconds.insert(centiseconds.len() - 2, '.');
                    centiseconds
                }));

                ui.add_space(100.0);

                if ui.add_enabled(!sfx.exists(), Button::new("Download")).clicked() {
                    let data = sfx.download(self.cdn_url.as_ref().unwrap());
                    if let Some(content) = data {
                        fs::write(sfx.path(), content).unwrap();
                    }
                }
                if ui.add_enabled(sfx.exists(), Button::new("Delete")).clicked() {
                    sfx.delete();
                }

                let audio_modal = Modal::new(ctx, "audio_placeholder");
                audio_modal.show(|ui| {
                    audio_modal.title(ui, "Audio");
                    audio_modal.frame(ui, |ui| {
                        audio_modal.body(ui, "This is supposed to play an audio.")
                    });
                    audio_modal.buttons(ui, |ui| {
                        if audio_modal.button(ui, "close").clicked() {
                            audio_modal.close()
                        }
                    });
                });

                if ui.button("Play").clicked() {
                    let data = sfx.download(self.cdn_url.as_ref().unwrap());

                    if let Some(_content) = data {
                        audio_modal.open()
                    }
                }
            });
        }
    }
}

impl GdSfx {
    pub fn run(self, options: NativeOptions) {
        eframe::run_native(
            "GDSFX",
            options,
            Box::new(|_cc| Box::new(self)),
        )
        .unwrap()
    }
}
