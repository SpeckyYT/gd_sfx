use eframe::egui::Ui;

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
    ([translations] $($lang:literal $($ident:ident)+)*) => {
        const TRANSLATIONS: [(&'static str, &[Credit]); 0 $(+ [$lang].len())* ] = [
            $(
                (
                    $lang,
                    &[
                        $($ident,)+
                    ],
                ),
            )*
        ];
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
    "en_GB"
        KR8GZ
    "de_AT"
        KR8GZ
    "en_US"
        SPECKY
    "it_IT"
        SPECKY
    "lld_BAD"
        SPECKY
    "tok_MP"
        SPECKY
    "nl_NL"
        TAGS
    "ua_UA"
        ELDYJ
    "rue_UA"
        ELDYJ
    "ru_RU"
        ELDYJ
    "pl_PL"
        GGOD
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

    if let Some((_, translators)) = TRANSLATIONS.iter().find(|(lang_name, _)| lang_name == &&rust_i18n::locale()) {
        if !translators.is_empty() {
            ui.label(t!("credits.this_project.translations", lang = util::format_locale(&rust_i18n::locale())));
            for (name, link) in translators.iter() {
                ui.hyperlink_to(*name, *link);
            }
        }
    }
}
