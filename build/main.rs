mod credits;
mod i18n;
mod locale_schema;
mod sfx_list;

mod util;

fn main() {
    credits::build();
    i18n::build();
    locale_schema::build();
    sfx_list::build();

    // rerun if any file in the locales folder changes
    util::get_locale_files()
        .map(|file| file.path())
        .for_each(build_script::cargo_rerun_if_changed);

    // rerun if the locales folder itself is changed (e.g. adding a new locale)
    build_script::cargo_rerun_if_changed(util::LOCALES_DIR);
}
