use eframe::egui::Ui;
use gdsfx_library::{MusicLibrary, SfxLibrary};
use crate::{layout, backend::{AppState, LibraryPage}};

// this build output file contains the following function:
// ```
// fn get_link(name: &str) -> Option<&str> { ... }
// ```
// see gdsfx-app/build/credits.rs
gdsfx_build::get_output!(include!("credits.rs"));

const DEVELOPERS: &[&str] = &["Specky", "kr8gz", "tags"];

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    ui.heading(t!("credits.sfx"));

    ui.add_space(10.0);

    match app_state.library_page {
        LibraryPage::Sfx => {
            for credits in sfx_library.credits() {
                ui.hyperlink_to(&credits.name, &credits.link);
            }
        },
        LibraryPage::Music => {
            for credits in &music_library.credits {
                let links = [
                    credits.url.as_ref(),
                    credits.yt_url.as_ref(),
                ];

                let url = links.into_iter().find_map(|url| url);

                match url {
                    Some(url) => ui.hyperlink_to(&credits.name, url),
                    None => ui.label(&credits.name),
                };
            }
        },
    }

    ui.add_space(10.0);

    ui.separator();

    ui.add_space(10.0);

    ui.heading(t!("credits.this_project"));
    ui.hyperlink_to("GitHub", "https://github.com/SpeckyYT/gd_sfx");

    ui.add_space(10.0);

    ui.label(t!("credits.this_project.developers"));

    for &developer in DEVELOPERS {
        add_optional_link(ui, developer);
    }

    ui.add_space(10.0);

    let current_locale = rust_i18n::locale();
    let translators = crate::get_translators(&current_locale);

    if !translators.is_empty() {
        ui.label(t!("credits.this_project.translations", lang = t!("language.name")));
        for &translator in translators {
            add_optional_link(ui, translator);
        }
    }
}

fn add_optional_link(ui: &mut Ui, name: &str) {
    if let Some(link) = get_link(name) {
        ui.hyperlink_to(name, link);
    } else {
        ui.label(name);
    }
}
