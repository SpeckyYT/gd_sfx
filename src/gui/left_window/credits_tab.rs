use eframe::egui::Ui;

use crate::gui::GdSfx;

// this generates the implementations of the functions below at compile-time
// (see build/credits.rs)
include!(concat!(env!("OUT_DIR"), "/credits.rs"));

fn link(name: &str) -> &str {
    __get_link(name)
}

fn translators(locale: &str) -> &[&str] {
    __get_translators(locale)
}

const DEVELOPERS: &[&str] = &["Specky", "kr8gz", "tags"];

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
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

    for &developer in DEVELOPERS {
        ui.hyperlink_to(developer, link(developer));
    }

    ui.add_space(10.0);

    let current_locale = rust_i18n::locale();

    let translators = translators(&current_locale);

    if !translators.is_empty() {
        ui.label(t!("credits.this_project.translations", lang = t!("language.name")));
        for &translator in translators {
            ui.hyperlink_to(translator, link(translator));
        }
    }
}
