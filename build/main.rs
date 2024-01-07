use std::path::Path;

mod lang_schema;
// mod lang_credits;
mod i18n;

mod util;

fn main() {
    lang_schema::build();
    // lang_credits::build();
    i18n::build();

    // rerun if any file in the lang folder changes
    Path::new("lang")
        .read_dir().unwrap()
        .flatten()
        .map(|entry| entry.path())
        .for_each(build_script::cargo_rerun_if_changed);
}
