use quote::quote;

use crate::util;

const OUTPUT_FILE: &str = "i18n.rs";

pub fn build() {
    let locales_dir = util::LOCALES_DIR;

    let tokens = quote! {
        #[macro_use]
        extern crate rust_i18n;

        i18n!(#locales_dir, fallback = "en_US");
    };

    // write macro invocation to OUTPUT_FILE to include!() it in main.rs
    // so that it updates every time the build script is rerun
    util::write_output_file(OUTPUT_FILE, tokens);
}
