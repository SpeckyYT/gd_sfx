use eframe::{egui::{RichText, Ui}, epaint::Color32};
use itertools::Itertools;

use library::{MusicLibrary, SfxLibrary};

use crate::{backend::{AppState, LibraryPage}, i18n::LocalizedEnum, layout};

const DEVELOPERS: &[&str] = &["Specky", "kr8gz", "tags"];
const CREDIT_LINKS: fn(&str) -> Option<&str> = build::get_output!(include!("credits/links.rs"));
const THEME_CREDITS: fn(&str) -> Option<&str> = build::get_output!(include!("credits/themes.rs"));
const TRANSLATORS: fn(&str) -> &[&str] = build::get_output!(include!("credits/translators.rs"));

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    ui.heading(t!(format!("credits.{}", app_state.library_page.localization_key())));

    ui.add_space(10.0);

    add_library_credits(ui, app_state, sfx_library, music_library);

    ui.add_space(10.0);

    ui.separator();

    ui.add_space(10.0);

    add_theme_credits(ui, app_state);

    ui.heading(t!("credits.this_project"));

    ui.add_space(5.0);

    ui.hyperlink_to("GitHub", "https://github.com/SpeckyYT/gd_sfx");

    ui.add_space(10.0);

    add_developers(ui);

    ui.add_space(10.0);

    add_translators(ui);

    ui.label(RichText::new(t!("credits.this_project.disaffiliation")).color(Color32::KHAKI));
}

fn add_link(ui: &mut Ui, name: &str) {
    if let Some(link) = CREDIT_LINKS(name) {
        ui.hyperlink_to(name, link);
    } else {
        ui.label(name);
    }
}

fn add_library_credits(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    match app_state.library_page {
        LibraryPage::Sfx => {
            let credits: Vec<_> = sfx_library.credits()
                .iter()
                .sorted_unstable_by(|a, b| a.name.cmp(&b.name))
                .collect();

            for credit in credits {
                ui.hyperlink_to(&credit.name, &credit.link);
            }
        },
        LibraryPage::Music => {
            let credits: Vec<_> = music_library.credits.values()
                .sorted_unstable_by(|a, b| a.name.cmp(&b.name))
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
}

fn add_theme_credits(ui: &mut Ui, app_state: &mut AppState) {
    if let Some(theme_credit) = THEME_CREDITS(app_state.settings.theme.localization_key()) {
        ui.heading("Theme Credit");

        ui.add_space(10.0);

        ui.label(format!("{:?}", app_state.settings.theme));

        add_link(ui, theme_credit);

        ui.add_space(10.0);

        ui.separator();

        ui.add_space(10.0);
    };
}

fn add_developers(ui: &mut Ui) {
    ui.label(t!("credits.this_project.developers"));

    for &developer in DEVELOPERS {
        add_link(ui, developer);
    }
}

fn add_translators(ui: &mut Ui) {
    let current_locale = rust_i18n::locale();
    let translators = TRANSLATORS(&current_locale);

    if translators.is_empty() { return }

    ui.label(t!("credits.this_project.translations", lang = t!("language.name")));

    for &translator in translators {
        add_link(ui, translator);
    }

    ui.add_space(10.0);
}
