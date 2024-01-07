use eframe::egui::Ui;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::{gui::GdSfx, util};

type Credit = (&'static str, &'static str);

macro_rules! credits {
    ([define] $($ident:ident => $name:literal: $link:literal )*) => {
        $(
            const $ident: Credit = ($name, $link);
        )*
    };
    ([developers] $($ident:ident)*) => {
        const DEVELOPERS: [Credit; 0 $(+ [$ident].len())* ] = [ $($ident,)* ];
    };
    ([translations] $($person:ident $($language:literal)+)*) => {
        lazy_static!{
            pub static ref TRANSLATIONS: HashMap<String, Vec<Credit>> = {
                let mut map = HashMap::new();

                // despite looking inefficient, this only takes around 25µs
                // so I don't think it's an issue
                $(
                    $(
                        map.entry($language.to_string()).or_insert(vec![]).push($person);
                    )+
                )*

                map
            };
        }
    };
}

credits!(
    [define]

    SPECKY => "Specky": "https://github.com/SpeckyYT"
    TAGS => "tags": "https://github.com/zTags"
    KR8GZ => "kr8gz": "https://github.com/kr8gz"
    ELDYJ => "eldyj": "https://github.com/eldyj"
    GGOD => "ggod": "https://github.com/GGodPL"
);

credits!(
    [developers]

    SPECKY
    TAGS
    KR8GZ
);

credits!(
    [translations]

    SPECKY
        "en_US"
        "it_IT"
        "lld_BAD"
        "tok_MP"
    KR8GZ
        "de_AT"
        "en_GB"
    TAGS
        "nl_NL"
    ELDYJ
        "ua_UA"
        "rue_UA"
        "ru_RU"
    GGOD
        "pl_PL"
);

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

    for (name, link) in DEVELOPERS {
        ui.hyperlink_to(name, link);
    }

    ui.add_space(10.0);


    // TODO "translators": [{"name": string, "link": string}] in lang jsons (add what i had to schema)
    // insert code for getting translation credits with OUT_DIR/i18n.rs when generating lang_schema

    let current_locale = rust_i18n::locale();

    if let Some(translators) = TRANSLATIONS.get(&current_locale) {
        ui.label(t!("credits.this_project.translations", lang = util::format_locale(&current_locale)));
        for (name, link) in translators {
            ui.hyperlink_to(*name, *link);
        }
    }
}
