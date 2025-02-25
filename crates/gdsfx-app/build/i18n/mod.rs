use quote::quote;

use gdsfx_shared::paths::LOCALES_DIR;

mod locale_schema;

pub fn build() {
    // rerun when files are added or removed
    build::rerun_if_changed(LOCALES_DIR);

    // rerun when files are modified
    gdsfx_files::read_dir(LOCALES_DIR).unwrap()
        .map(|file| file.path())
        .for_each(build::rerun_if_changed);
    
    gdsfx_build::write_output_rust("i18n.rs", quote! {
        // forces this proc macro to rerun when modified
        rust_i18n::i18n!(#LOCALES_DIR, fallback = "en_US");
    });

    // generate updated locale schema
    locale_schema::build();
}
