use quote::quote;

mod locale_schema;

pub fn build() {
    use files::paths::LOCALES_DIR;

    // files added or removed
    build::cargo_rerun_if_changed(LOCALES_DIR);
    // files modified
    files::read_dir(LOCALES_DIR).unwrap()
        .map(|file| file.path())
        .for_each(build::cargo_rerun_if_changed);

    build::write_output_rust("i18n.rs", quote! {
        // if the build script reruns, it forces this proc macro to rerun too
        rust_i18n::i18n!(#LOCALES_DIR, fallback = "en_US");
    });

    // generate new locale schema whenever locales are changed
    locale_schema::build();
}
