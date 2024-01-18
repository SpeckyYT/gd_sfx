use ahash::{HashMap, HashMapExt};

use gdsfx_build::TokenStream;
use gdsfx_files::paths;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct Locale {
    #[serde(rename = "language.translators")]
    translators: Vec<String>,
}

pub fn build() -> TokenStream {
    let mut translations = HashMap::new();
    let locale_files = gdsfx_files::read_dir(paths::build::LOCALES_DIR).unwrap();

    for file in locale_files {
        let path = file.path();
        let Some(locale) = path.file_stem().and_then(|path| path.to_str()) else { continue };

        let locale_json: Locale = gdsfx_files::read_json_file(&path).unwrap();            
        let translators = locale_json.translators;

        translations.insert(locale.to_string(), quote!(&[ #(#translators),* ]));
    }

    let locales = translations.keys();
    let translators = translations.values();

    quote! {
        fn get_translators(locale: &str) -> &[&str] {
            match locale {
                #(#locales => #translators,)*
                _ => &[]
            }
        }
    }
}
