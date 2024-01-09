use std::{ffi::OsStr, collections::HashMap};

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

use crate::util;

const CREDITS_FILE: &str = "credits.json";
const OUTPUT_FILE: &str = "credits.rs";

type Credits = HashMap<String, String>;

#[derive(Deserialize)]
struct Locale {
    #[serde(rename = "language.translators")]
    translators: Vec<String>,
}

pub fn build() {
    let mut tokens = TokenStream::new();
    
    tokens.extend(generate_link_fn());
    tokens.extend(generate_translators_fn());

    util::write_output_file(OUTPUT_FILE, tokens);

    build_script::cargo_rerun_if_changed(CREDITS_FILE);
}

fn generate_link_fn() -> TokenStream {
    let credits_json: Credits = util::read_json_file(CREDITS_FILE);

    let names = credits_json.keys();
    let links = credits_json.values();

    quote! {
        fn __get_link(name: &str) -> Option<&str> {
            match name {
                #(#names => Some(#links),)*
                _ => None
            }
        }
    }
}

fn generate_translators_fn() -> TokenStream {
    let mut translations = HashMap::new();

    for file in util::get_locale_files().map(|file| file.path()) {
        if let Some(locale) = file.file_stem().and_then(OsStr::to_str) {
            let locale_json: Locale = util::read_json_file(&file);
            let translators = locale_json.translators;

            translations.insert(locale.to_string(), quote! {
                &[ #(#translators),* ]
            });
        }
    }

    let locales = translations.keys();
    let translators = translations.values();

    quote! {
        fn __get_translators(locale: &str) -> &[&str] {
            match locale {
                #(#locales => #translators,)*
                _ => unreachable!()
            }
        }
    }
}
