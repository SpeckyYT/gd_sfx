mod locale_schema;
mod i18n;

mod util;

fn main() {
    locale_schema::build();
    i18n::build();

    // rerun if any file in the locales folder changes
    util::get_locale_files()
        .map(|file| file.path())
        .for_each(build_script::cargo_rerun_if_changed);
}
