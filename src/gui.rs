use eframe::{
    egui::{self, Button, Ui},
    NativeOptions,
};
use pretty_bytes::converter::convert;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    audio::{play_sound, stop_audio},
    favourites::{add_favourite, has_favourite, remove_favourite},
    library::{Library, LibraryEntry},
    requests::CDN_URL,
    stats::EXISTING_SOUND_FILES,
    util::stringify_duration,
};

pub type VersionType = usize;

#[derive(Debug, Default, Clone)]
pub struct GdSfx {
    pub cdn_url: Option<String>,
    pub sfx_version: Option<VersionType>,
    pub sfx_library: Option<Library>,

    pub stage: Stage,
    pub search_query: String,
    pub sorting: Sorting,
    pub selected_sfx: Option<LibraryEntry>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Stage {
    #[default]
    Library,
    Favourites,
    Stats,
    Credits,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    #[default]
    Default,
    NameInc,   // a - z
    NameDec,   // z - a
    LengthInc, // 0.00 - 1.00
    LengthDec, // 1.00 - 0.00
    IdInc,     // 9 - 0
    IdDec,     // 0 - 9
    SizeInc,   // 0kb - 9kb
    SizeDec,   // 9kb - 0kb
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
            Stage::iter().for_each(|stage| {
                ui.selectable_value(&mut gdsfx.stage, stage, format!("{:?}", stage));
            });
        });
        ui.add_space(2.0);
    });
}

fn main_scroll_area(ctx: &egui::Context, gdsfx: &mut GdSfx) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        /*
        // reconsider these
        if let Some(version) = gdsfx.sfx_version {
            ui.heading(format!("Library version: {version}"));
        }
        if ui.button("Force-update library").clicked() {
            gdsfx.get_sfx_library(true);
        }
        ui.separator();
        */

        if let Stage::Library | Stage::Favourites = gdsfx.stage {
            search_bar(ui, gdsfx);
            sort_menu(ui, gdsfx);
            ui.separator();
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(sfx_library) = gdsfx.sfx_library.as_ref() {
                match gdsfx.stage {
                    Stage::Library => {
                        let library = sfx_library.sound_effects.clone();
                        let filter_str = &gdsfx.search_query.to_ascii_lowercase();
                        if let Some(mut sfx) = filter_sounds(library, filter_str) {
                            remove_empty_category_nodes(&mut sfx);
                            library_list(ui, gdsfx, &sfx);
                        }
                    }
                    Stage::Favourites => {
                        favourites_list(ui, gdsfx, sfx_library.sound_effects.clone())
                    }
                    Stage::Stats => stats_list(ui, gdsfx),
                    Stage::Credits => credits_list(ui, gdsfx),
                }
            }
        });
    });
}

fn library_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: &LibraryEntry) {
    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut egui::Ui) {
        // let q = gdsfx.search_query.to_ascii_lowercase();
        match entry {
            LibraryEntry::Category { children, .. } => {
                let (mut categories, mut sounds): (Vec<_>, Vec<_>) =
                    children.iter().partition(|a| a.is_category());

                let sorting = |a: &&LibraryEntry, b: &&LibraryEntry| {
                    match gdsfx.sorting {
                        Sorting::Default => std::cmp::Ordering::Equal,
                        Sorting::NameInc => a.name().cmp(b.name()),
                        Sorting::NameDec => b.name().cmp(a.name()),
                        Sorting::LengthInc => a.duration().cmp(&b.duration()),
                        Sorting::LengthDec => b.duration().cmp(&a.duration()),
                        Sorting::IdInc => b.id().cmp(&a.id()), // this is not a bug, in gd, the id sorting is reversed,
                        Sorting::IdDec => a.id().cmp(&b.id()), // in-game it's `ID+ => 9 - 0; ID- => 0 - 9`
                        Sorting::SizeInc => a.bytes().cmp(&b.bytes()),
                        Sorting::SizeDec => b.bytes().cmp(&a.bytes()),
                    }
                };

                categories.sort_by(sorting);

                if entry.parent() == 0 {
                    // root
                    for child in categories {
                        recursive(gdsfx, child, ui);
                    }
                } else {
                    sounds.sort_by(sorting);

                    // let is_disabled = sounds.is_empty() && !q.is_empty();
                    // ui.set_enabled(!is_disabled);

                    ui.collapsing(entry.name(), |ui| {
                        for child in categories {
                            recursive(gdsfx, child, ui);
                        }
                        for child in sounds {
                            recursive(gdsfx, child, ui);
                        }
                    });

                    ui.set_enabled(true);
                }
            }
            LibraryEntry::Sound { .. } => {
                sfx_button(ui, gdsfx, entry);
            }
        }
    }
    recursive(gdsfx, sfx_library, ui);
}

fn favourites_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: LibraryEntry) {
    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut egui::Ui) {
        match entry {
            LibraryEntry::Category { children, .. } => {
                for child in children {
                    recursive(gdsfx, child, ui);
                }
            }
            LibraryEntry::Sound { name, id, .. } => {
                if has_favourite(*id)
                    && name
                        .to_ascii_lowercase()
                        .contains(&gdsfx.search_query.to_ascii_lowercase())
                {
                    sfx_button(ui, gdsfx, entry)
                }
            }
        }
    }
    recursive(gdsfx, &sfx_library, ui);
}

fn stats_list(ui: &mut Ui, gdsfx: &mut GdSfx) {
    // (bytes, duration, files)
    fn recursive(entry: &LibraryEntry) -> (u128, u128, i64) {
        match entry {
            LibraryEntry::Category { children, .. } => children
                .iter()
                .map(recursive)
                .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                .unwrap_or((0, 0, 1)),
            LibraryEntry::Sound {
                bytes, duration, ..
            } => (*bytes as u128, *duration as u128, 1),
        }
    }
    let (total_bytes, total_duration, total_files) =
        recursive(&gdsfx.sfx_library.as_ref().unwrap().sound_effects);

    ui.heading("SFX Library");

    ui.add_space(10.0);

    ui.label(format!("Total files: {}", total_files));
    ui.label(format!(
        "Total size: {}",
        pretty_bytes::converter::convert(total_bytes as f64)
    ));
    ui.label(format!(
        "Total duration: {}s",
        stringify_duration(total_duration as i64)
    ));

    ui.add_space(30.0);

    ui.heading("SFX Files");

    ui.add_space(10.0);

    ui.label(format!(
        "Downloaded sfx files: {}",
        EXISTING_SOUND_FILES.lock().unwrap().len()
    ));
}

fn credits_list(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading("SFX Credits");
    ui.add_space(10.0);
    for credits in &gdsfx.sfx_library.as_ref().unwrap().credits {
        ui.hyperlink_to(&credits.name, &credits.link);
    }

    ui.add_space(30.0);

    ui.heading("<This project>");
    ui.hyperlink_to("GitHub", "https://github.com/SpeckyYT/gd_sfx");
    ui.add_space(10.0);

    for (name, link) in [
        ("Specky", "https://github.com/SpeckyYT"),
        ("tags", "https://github.com/zTags"),
        ("kr8gz", "https://github.com/kr8gz"),
    ] {
        ui.hyperlink_to(name, link);
    }
}

fn search_bar(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading("Search");
    ui.text_edit_singleline(&mut gdsfx.search_query);
}

fn sort_menu(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.menu_button("Sorting", |ui| {
        for (alternative, text) in [
            (Sorting::Default, "Default"),
            (Sorting::NameInc, "Name+"),
            (Sorting::NameDec, "Name-"),
            (Sorting::LengthInc, "Length+"),
            (Sorting::LengthDec, "Length-"),
            (Sorting::IdInc, "ID+"),
            (Sorting::IdDec, "ID-"),
            (Sorting::SizeInc, "Size+"),
            (Sorting::SizeDec, "Size-"),
        ] {
            let response = ui.radio_value(&mut gdsfx.sorting, alternative, text);
            if response.clicked() {
                ui.close_menu();
            }
        }
    });
}

fn sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: &LibraryEntry) {
    let sound = ui.button(entry.pretty_name());
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

            ui.add_space(25.0);

            ui.code(sfx.get_string());

            ui.add_space(25.0);

            ui.heading(format!("ID: {}", sfx.id()));
            ui.heading(format!("Category ID: {}", sfx.parent()));
            ui.heading(format!("Size: {}", convert(sfx.bytes() as f64)));
            ui.heading(format!("Duration: {}s", stringify_duration(sfx.duration())));

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

fn remove_empty_category_nodes(node: &mut LibraryEntry) -> bool {
    match node {
        LibraryEntry::Sound { .. } => true,
        LibraryEntry::Category { children, .. } => {
            // Recursively check for any sounds
            children.iter_mut().any(remove_empty_category_nodes)
        }
    }
}

fn filter_sounds(entry: LibraryEntry, filter_str: &str) -> Option<LibraryEntry> {
    match entry {
        LibraryEntry::Sound { ref name, .. } => {
            name.to_ascii_lowercase().contains(filter_str).then_some(entry)
        }
        LibraryEntry::Category { id, name, parent, children } => {
            // Recursively filter sounds in subcategories
            let filtered_sounds: Vec<LibraryEntry> = children
                .into_iter()
                .flat_map(|node| filter_sounds(node, filter_str))
                .collect();

            // Only keep the category if it contains any filtered sounds
            let has_children = !filtered_sounds.is_empty();
            has_children.then_some(LibraryEntry::Category { id, name, parent, children: filtered_sounds })
        }
    }
}
