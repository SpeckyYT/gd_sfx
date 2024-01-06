use eframe::egui::Ui;

use crate::{gui::GdSfx, util};

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

    for (name, link) in [
        ("Specky", "https://github.com/SpeckyYT"),
        ("tags", "https://github.com/zTags"),
        ("kr8gz", "https://github.com/kr8gz"),
    ] {
        ui.hyperlink_to(name, link);
    }

    ui.add_space(10.0);

    ui.label(t!("credits.this_project.translations", lang = util::format_locale(&rust_i18n::locale())));

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