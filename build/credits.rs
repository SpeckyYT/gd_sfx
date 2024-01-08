use std::{ffi::OsStr, collections::HashMap};

use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;

use crate::util;

const CREDITS_FILE: &str = "credits.json";
const OUTPUT_FILE: &str = "credits.rs";

// TODO: actually use serde for its intended purpose (deserializing)
pub fn build() {
    let mut tokens = TokenStream::new();
    
    tokens.extend(generate_link_fn());
    tokens.extend(generate_translators_fn());

    util::write_output_file(OUTPUT_FILE, tokens);
}

fn generate_link_fn() -> TokenStream {
    let credits_json = util::read_json_file(CREDITS_FILE);

    let credits_json = credits_json
        .as_object()
        .unwrap_or_else(|| panic!("JSON in file {CREDITS_FILE:?} is not an object"));

    let credit_names = credits_json.keys();
    let credit_links = credits_json.values().map(|link| {
        link.as_str().unwrap_or_else(|| panic!("Expected a string containing an URL, found {link}"))
    });

    quote! {
        fn __get_link(name: &str) -> &str {
            match name {
                #(#credit_names => #credit_links,)*
                _ => unreachable!()
            }
        }
    }
}

fn generate_translators_fn() -> TokenStream {
    let mut translations = HashMap::new();

    for file in util::get_locale_files().map(|file| file.path()) {
        if let Some(locale) = file.file_stem().and_then(OsStr::to_str) {
            let locale_json = util::read_json_file(&file);

            let translators = locale_json
                .get("language.translators")
                .and_then(Value::as_array)
                .unwrap_or_else(|| panic!("Locale \"{locale}\" does not contain a \"language.translators\" array"))
                .iter()
                .map(|value| value.as_str().unwrap_or_else(|| panic!("Invalid entry in \"language.translators\" in locale {locale}")))
                .collect::<Vec<_>>();

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
