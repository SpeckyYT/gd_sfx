use std::fs;

use quote::quote;

use crate::util;

const OUTPUT_PATH: &str = "i18n.rs";

pub fn build() {
    let locales_dir = util::LOCALES_DIR;

    let test = quote! {
        #[macro_use]
        extern crate rust_i18n;

        i18n!(#locales_dir, fallback = "en_US");
    };

    // write macro invocation to OUTPUT_PATH to include!() it in main.rs
    // so that it updates every time the build script is rerun
    fs::write(util::get_output_file(OUTPUT_PATH), test.to_string())
        .unwrap_or_else(|e| panic!("Couldn't write to file '{OUTPUT_PATH}': {e}"));
}
