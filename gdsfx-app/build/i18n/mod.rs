use quote::quote;

mod locale_schema;

pub const LOCALES_DIR: &str = gdsfx_files::workspace_path!("locales");

pub fn build() {
    // files added or removed
    build_script::cargo_rerun_if_changed(LOCALES_DIR);
    // files modified
    gdsfx_files::read_dir(LOCALES_DIR).unwrap()
        .map(|file| file.path())
        .for_each(build_script::cargo_rerun_if_changed);

    gdsfx_build::write_output_rust("i18n.rs", quote! {
        // if the build script reruns, it forces this proc macro to rerun too
        rust_i18n::i18n!(#LOCALES_DIR, fallback = "en_US");
    });

    // generate new locale schema whenever locales are changed
    locale_schema::build();
}
