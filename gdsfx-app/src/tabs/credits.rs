use eframe::{egui::{RichText, Ui}, epaint::Color32};
use gdsfx_library::{MusicLibrary, SfxLibrary};
use itertools::Itertools;
use crate::{backend::{AppState, LibraryPage}, i18n::LocalizedEnum, layout};

// this build output file contains the following function:
// ```
// fn get_link(name: &str) -> Option<&str> { ... }
// ```
// see gdsfx-app/build/credits.rs
gdsfx_build::get_output!(include!("credits.rs"));

const DEVELOPERS: &[&str] = &["Specky", "kr8gz", "tags"];

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    ui.heading(t!(&format!("credits.{}", app_state.library_page.localization_key())));

    ui.add_space(10.0);

    match app_state.library_page {
        LibraryPage::Sfx => {
            let credits: Vec<_> = sfx_library.credits()
                .iter()
                .sorted_unstable_by(|a,b| a.name.cmp(&b.name))
                .collect();

            for credit in credits {
                ui.hyperlink_to(&credit.name, &credit.link);
            }
        },
        LibraryPage::Music => {
            let credits: Vec<_> = music_library.credits.values()
                .sorted_unstable_by(|a,b| a.name.cmp(&b.name))
                .collect();

            for credit in credits {
                let links = [
                    credit.url.as_ref(),
                    credit.yt_url.as_ref(),
                ];
                let url = links.into_iter().find_map(|url| url);
                match url {
                    Some(url) => ui.hyperlink_to(&credit.name, url),
                    None => ui.label(&credit.name),
                };
            }
        },
    }

    ui.add_space(10.0);

    ui.separator();

    ui.add_space(10.0);

    ui.heading(t!("credits.this_project"));

    ui.add_space(5.0);

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
        ui.add_space(10.0);
    }

    ui.label(RichText::new(t!("credits.this_project.disaffiliation")).color(Color32::KHAKI));
}

fn add_optional_link(ui: &mut Ui, name: &str) {
    if let Some(link) = get_link(name) {
        ui.hyperlink_to(name, link);
    } else {
        ui.label(name);
    }
}
