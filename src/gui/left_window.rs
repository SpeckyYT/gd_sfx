use std::{sync::{Arc, Mutex}, thread::JoinHandle};

use eframe::{egui::{Ui, Context, ProgressBar, ComboBox, SidePanel, ScrollArea}, epaint::Color32};
use egui_modal::Modal;
use lazy_static::lazy_static;

use crate::{library::LibraryEntry, settings::{SETTINGS, self, save}, util::{download_everything, delete_everything, format_locale, stringify_duration}, stats::EXISTING_SOUND_FILES, requests::CDN_URL, audio};

use super::{GdSfx, Tab};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    #[default]
    Default,
    NameInc,   // a - z
    NameDec,   // z - a
    LengthInc, // 0.00 - 1.00
    LengthDec, // 1.00 - 0.00
    IdInc,     // 0 - 9
    IdDec,     // 9 - 0
    SizeInc,   // 0kb - 9kb
    SizeDec,   // 9kb - 0kb
}

pub fn left_window(ctx: &Context, gdsfx: &mut GdSfx) {
    SidePanel::left("left_panel").show(ctx, |ui| {
        if let Tab::Library | Tab::Favourites = gdsfx.tab {
            search_bar(ui, gdsfx);
            sort_menu(ui, gdsfx);
            ui.separator();
        }

        ScrollArea::vertical().show(ui, |ui| {
            if let Some(sfx_library) = gdsfx.sfx_library.as_ref() {
                match gdsfx.tab {
                    // TODO this can be simplified
                    Tab::Library => {
                        let mut library = sfx_library.sound_effects.clone();
                        filter_sounds(gdsfx, &mut library);
                        library_list(ui, gdsfx, &library);
                    }
                    Tab::Favourites => {
                        let mut library = sfx_library.sound_effects.clone();
                        filter_sounds(gdsfx, &mut library);
                        favourites_list(ui, gdsfx, library);
                    }
                    Tab::Tools => tools_list(ui, gdsfx, ctx),
                    Tab::Settings => settings_list(ui, gdsfx),
                    Tab::Stats => stats_list(ui, gdsfx),
                    Tab::Credits => credits_list(ui, gdsfx),
                }
            }
        });
    });
}

pub fn library_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: &LibraryEntry) {
    fn recursive(gdsfx: &mut GdSfx, entry: &LibraryEntry, ui: &mut Ui) {
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
                        Sorting::IdInc => a.id().cmp(&b.id()),
                        Sorting::IdDec => b.id().cmp(&a.id()),
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

                    let enabled = entry.is_enabled();

                    let should_add = enabled || !SETTINGS.lock().unwrap().filter_search;

                    if should_add {
                        ui.add_enabled_ui(enabled, |ui| {
                            ui.collapsing(entry.name(), |ui| {
                                for child in children {
                                    recursive(gdsfx, child, ui);
                                }
                            });
                        });
                    }
                }
            }
            LibraryEntry::Sound { .. } => {
                sfx_button(ui, gdsfx, entry);
            }
        }
    }
    recursive(gdsfx, sfx_library, ui);
}

pub fn favourites_list(ui: &mut Ui, gdsfx: &mut GdSfx, sfx_library: LibraryEntry) {
    fn recursive(ui: &mut Ui, gdsfx: &mut GdSfx, entry: &LibraryEntry) {
        match entry {
            LibraryEntry::Category { children, .. } => {
                for child in children {
                    recursive(ui, gdsfx, child);
                }
            }
            LibraryEntry::Sound { id, .. } => {
                if settings::has_favourite(*id) {
                    sfx_button(ui, gdsfx, entry)
                }
            }
        }
    }
    recursive(ui, gdsfx, &sfx_library);
}

lazy_static! {
    // (done, to_do)
    pub static ref DOWNLOAD_PROGRESS: Arc<Mutex<(usize, usize)>> = Default::default();
    pub static ref DOWNLOAD_HANDLE: Arc<Mutex<Option<JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

pub fn tools_list(ui: &mut Ui, gdsfx: &mut GdSfx, ctx: &Context) {
    let modal_generator = |title, id_source| -> Modal {
        let modal = Modal::new(ctx, format!("{}_modal", id_source));

        modal.show(|ui| {
            let mut download_handle = DOWNLOAD_HANDLE.lock().unwrap();

            if let Some(handle) = download_handle.as_ref() {
                if handle.is_finished() {
                    *download_handle = None;
                    modal.close();
                }
            }
            drop(download_handle);

            modal.title(ui, title);
    
            let (done, todo) = *DOWNLOAD_PROGRESS.lock().unwrap();

            let progress_bar = ProgressBar::new(if todo == 0 { 0.0 } else { done as f32 / todo as f32 })
            .animate(true)
            .text(format!("{done} / {todo}"));

            ui.heading(t!("tools.progress"));

            ui.add(progress_bar);
        });

        modal
    };

    let download_modal = modal_generator(t!("tools.download_all_sfx.title"), "download");
    let delete_modal = modal_generator(t!("tools.delete_all_sfx.title"), "delete");
    
    ui.heading(t!("tools"));
    
    ui.add_space(10.0);

    ui.colored_label(Color32::RED, t!("tools.warning.long_time"));
    ui.colored_label(Color32::RED, t!("tools.warning.program_not_usable"));

    ui.add_space(10.0);

    ui.label(t!("tools.instruction"));

    ui.add_space(10.0);

    if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
        *DOWNLOAD_PROGRESS.lock().unwrap() = (0,0);
        download_modal.open();
        *DOWNLOAD_HANDLE.lock().unwrap() = Some(download_everything(gdsfx.sfx_library.as_ref().unwrap().sound_effects.clone()));
    }
    if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
        *DOWNLOAD_PROGRESS.lock().unwrap() = (0,0);
        delete_modal.open();
        *DOWNLOAD_HANDLE.lock().unwrap() = Some(delete_everything());
    }
}

pub fn settings_list(ui: &mut Ui, _gdsfx: &mut GdSfx) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);

    let mut settings = SETTINGS.lock().unwrap();
    let initial_settings = *settings;

    ui.checkbox(&mut settings.filter_search, t!("settings.hide_empty_categories"));

    let mut current_locale = rust_i18n::locale();
    let initial_locale = current_locale.clone();
    ComboBox::from_label(t!("settings.language"))
        .selected_text(format_locale(&current_locale))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                ui.selectable_value(&mut current_locale, locale.to_string(), format_locale(locale));
            }
        });

    rust_i18n::set_locale(&current_locale);

    if *settings != initial_settings || current_locale != initial_locale {
        drop(settings); // fixes deadlock (geometry dash reference)
        save();
    }
}

pub fn stats_list(ui: &mut Ui, gdsfx: &mut GdSfx) {
    struct Stats {
        bytes: u128,
        duration: u128,
        files: i64,
    }

    fn recursive(entry: &LibraryEntry) -> Stats {
        match entry {
            LibraryEntry::Category { children, .. } => children
                .iter()
                .map(recursive)
                .reduce(|a, b| Stats {
                    bytes: a.bytes + b.bytes,
                    duration: a.duration + b.duration,
                    files: a.files + b.files
                })
                .unwrap_or(Stats { bytes: 0, duration: 0, files: 1 }),

            LibraryEntry::Sound { bytes, duration, .. } => Stats {
                bytes: *bytes as u128,
                duration: *duration as u128,
                files: 1
            }
        }
    }

    let Stats { bytes, duration, files } = recursive(&gdsfx.sfx_library.as_ref().unwrap().sound_effects);

    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    ui.label(t!("stats.library.files", files = files));
    ui.label(t!("stats.library.size", size = pretty_bytes::converter::convert(bytes as f64)));
    ui.label(t!("stats.library.duration", duration = stringify_duration(duration as i64)));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    ui.label(t!("stats.files.downloaded", files = EXISTING_SOUND_FILES.lock().unwrap().len()));
}

pub fn credits_list(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading(t!("credits.sfx"));

    ui.add_space(10.0);

    for credits in &gdsfx.sfx_library.as_ref().unwrap().credits {
        ui.hyperlink_to(&credits.name, &credits.link);
    }

    ui.add_space(20.0);

    ui.heading(t!("credits.this_project"));
    ui.hyperlink_to("GitHub", "https://github.com/SpeckyYT/gd_sfx");

    ui.add_space(10.0);

    ui.label(t!("credits.this_project.developers"));

    for (name, link) in [
        ("Specky", "https://github.com/SpeckyYT"),
        ("tags", "https://github.com/zTags"),
        ("kr8gz", "https://github.com/kr8gz"),
    ] {
        ui.hyperlink_to(name, link);
    }

    ui.add_space(10.0);

    ui.label(t!("credits.this_project.translations", lang = format_locale(&rust_i18n::locale())));

    match rust_i18n::locale().as_str() {
        "de_AT" => {
            ui.hyperlink_to("kr8gz", "https://github.com/kr8gz");
        }
        "it_IT" | "lld_BAD" | "tok_MP" => {
            ui.hyperlink_to("Specky", "https://github.com/SpeckyYT");
        }
        "nl_NL" => {
            ui.hyperlink_to("tags", "https://github.com/zTags");
        }
        "ua_UA" => {
            ui.hyperlink_to("eldyj", "https://github.com/eldyj");
        }
        "pl_PL" => {
            ui.hyperlink_to("ggod", "https://github.com/GGodPL");
        }
        _ => {}
    }
}


fn search_bar(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading(t!("search"));
    ui.text_edit_singleline(&mut gdsfx.search_query);
}

fn sort_menu(ui: &mut Ui, gdsfx: &mut GdSfx) {
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
}

fn filter_sounds(gdsfx: &mut GdSfx, node: &mut LibraryEntry) {
    match node {
        LibraryEntry::Sound { .. } => {
            node.set_enabled(gdsfx.matches_query(node));
        }
        LibraryEntry::Category { children, .. } => {
            // Recursively filter sounds in subcategories
            children.iter_mut().for_each(|child| filter_sounds(gdsfx, child));

            let any_enabled = children.iter().any(LibraryEntry::is_enabled);
            node.set_enabled(any_enabled);
        }
    }
}

pub fn sfx_button(ui: &mut Ui, gdsfx: &mut GdSfx, entry: &LibraryEntry) {
    if !entry.is_enabled() { return }

    let sound = ui.button(entry.pretty_name());

    if sound.hovered() {
        gdsfx.selected_sfx = Some(entry.clone());
    }

    if sound.clicked() {
        audio::stop_audio();
        audio::play_sound(entry, CDN_URL);
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
}
