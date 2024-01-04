use eframe::{
    egui::{self, Button, Ui},
    NativeOptions,
};
use pretty_bytes::converter::convert;

use crate::{
    audio::{play_sound, stop_audio},
    favourites::{add_favourite, has_favourite, remove_favourite},
    library::LibraryEntry,
    requests::CDN_URL,
};

pub type VersionType = usize;

#[derive(Debug, Default, Clone)]
pub struct GdSfx {
    pub cdn_url: Option<String>,
    pub sfx_version: Option<VersionType>,
    pub sfx_library: Option<LibraryEntry>,

    pub stage: Stage,
    pub selected_sfx: Option<LibraryEntry>,
    pub search_query: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Library,
    Favourites,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top_panel(ctx, self);
        main_scroll_area(ctx, self);
        side_bar_sfx(ctx, self.selected_sfx.as_ref());
    }
}

impl GdSfx {
    pub fn run(self, options: NativeOptions) {
        eframe::run_native("GDSFX", options, Box::new(|_cc| Box::new(self))).unwrap()
    }
}

fn top_panel(ctx: &egui::Context, gdsfx: &mut GdSfx) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut gdsfx.stage, Stage::Library, "Library");
            ui.selectable_value(&mut gdsfx.stage, Stage::Favourites, "Favourites");
        });
        ui.add_space(2.0);
    });
}

fn main_scroll_area(ctx: &egui::Context, gdsfx: &mut GdSfx) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        if let Some(version) = gdsfx.sfx_version {
            ui.heading(format!("Library version: {version}"));
        }
        if ui.button("Force-update library").clicked() {
            gdsfx.get_sfx_library(true);
        }
        ui.add_space(10.0);
        ui.text_edit_singleline(&mut gdsfx.search_query);
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(sfx_library) = gdsfx.sfx_library.as_ref() {
                match gdsfx.stage {
                    Stage::Library => library_list(ui, gdsfx, sfx_library.clone()),
                    Stage::Favourites => favourites_list(ui, gdsfx, sfx_library.clone()),
                }
            }
        });
    });
}

fn library_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: LibraryEntry) {
    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut egui::Ui, is_root: bool) {
        let q = gdsfx.search_query.clone().to_ascii_lowercase();
        match entry {
            LibraryEntry::Category { children, .. } => {
                if is_root {
                    for child in children {
                        recursive(gdsfx, child, ui, false);
                    }
                } else {
                    let filtered_children = children
                        .iter()
                        .filter(|x| x.name().to_ascii_lowercase().contains(&q))
                        .collect::<Vec<_>>();
                    if filtered_children.is_empty() {
                        ui.set_enabled(false);
                    }
                    ui.collapsing(entry.name(), |sub_ui| {
                        for child in filtered_children {
                            recursive(gdsfx, child, sub_ui, false);
                        }
                    });
                }
            }
            LibraryEntry::Sound { .. } => {
                sfx_button(ui, gdsfx, entry);
            }
        }
    }
    recursive(gdsfx, &sfx_library, ui, true);
}

fn favourites_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: LibraryEntry) {
    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut egui::Ui) {
        match entry {
            LibraryEntry::Category { children, .. } => {
                for child in children {
                    recursive(gdsfx, child, ui);
                }
            }
            LibraryEntry::Sound { id, .. } => {
                if has_favourite(*id) {
                    sfx_button(ui, gdsfx, entry)
                }
            }
        }
    }
    recursive(gdsfx, &sfx_library, ui);
}

fn sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: &LibraryEntry) {
    let sound = ui.button(entry.name());
    if sound.hovered() {
        gdsfx.selected_sfx = Some(entry.clone());
    }
    if sound.clicked() {
        stop_audio();
        play_sound(entry, CDN_URL);
    }
    sound.context_menu(|ui| {
        if has_favourite(entry.id()) {
            if ui.button("Remove favourite").clicked() {
                remove_favourite(entry.id());
                ui.close_menu();
            }
        } else if ui.button("Favourite").clicked() {
            add_favourite(entry.id());
            ui.close_menu();
        }
        if entry.exists() {
            if ui.button("Delete").clicked() {
                entry.delete();
                ui.close_menu();
            }
        } else if ui.button("Download").clicked() {
            entry.download_and_store();
            ui.close_menu();
        }
    });
}

fn side_bar_sfx(ctx: &egui::Context, sfx: Option<&LibraryEntry>) {
    if let Some(sfx) = sfx {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(sfx.name());

            ui.add_space(50.0);

            ui.collapsing("Original code", |ui| {
                ui.code(sfx.get_string());
            });
            ui.heading(format!("ID: {}", sfx.id()));
            ui.heading(format!("Category ID: {}", sfx.parent()));
            ui.heading(format!("Size: {}", convert(sfx.bytes() as f64)));
            ui.heading(format!("Duration: {}s", {
                let mut centiseconds = format!("{:>03}", sfx.duration());
                centiseconds.insert(centiseconds.len() - 2, '.');
                centiseconds
            }));

            ui.add_space(50.0);

            if ui
                .add_enabled(!sfx.exists(), Button::new("Download"))
                .clicked()
            {
                sfx.download_and_store();
            }
            if ui
                .add_enabled(sfx.exists(), Button::new("Delete"))
                .clicked()
            {
                sfx.delete();
            }
            if ui.button("Play").clicked() {
                play_sound(sfx, CDN_URL);
            }
            if ui.button("Stop").clicked() {
                stop_audio();
            }
        });
    }
}
