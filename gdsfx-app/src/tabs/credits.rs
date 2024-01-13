use eframe::egui::Ui;

use crate::GdSfx;

// this build output file contains the following function:
// ```
// fn get_link(name: &str) -> Option<&str> { ... }
// ```
// see gdsfx-app/build/credits.rs
gdsfx_build::include!("credits.rs");

const DEVELOPERS: &[&str] = &["Specky", "kr8gz", "tags"];

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    ui.heading(t!("credits.sfx"));

    ui.add_space(10.0);

    for credits in gdsfx.library.get_credits() {
        ui.hyperlink_to(&credits.name, &credits.link);
    }

    ui.add_space(20.0);

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
