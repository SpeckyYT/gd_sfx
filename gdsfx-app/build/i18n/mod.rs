use gdsfx_build::TokenStream;
use gdsfx_files::paths;

mod update;
mod translators;
mod locale_schema;

const OUTPUT_FILE: &str = "i18n.rs";

pub fn build() {
    // if a file is added to or removed from the locales directory...
    gdsfx_build::cargo_rerun_if_changed(paths::build::LOCALES_DIR);
    // ...or any file in it is changed
    gdsfx_files::read_dir(paths::build::LOCALES_DIR).unwrap()
        .map(|file| file.path())
        .for_each(gdsfx_build::cargo_rerun_if_changed);

    let mut tokens = TokenStream::new();

    tokens.extend(update::build());
    tokens.extend(translators::build());

    gdsfx_build::write_output_rust(OUTPUT_FILE, tokens);

    // also generate new locale schema whenever a locale changes
    locale_schema::build();
}
