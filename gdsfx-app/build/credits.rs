use ahash::HashMap;
use quote::{quote, ToTokens};
use serde::Deserialize;
use gdsfx_build::TokenStream;

pub fn build() {
    generate_from_json(gdsfx_files::workspace_path!("credits/links.json"), "credits/links.rs");
    generate_from_json(gdsfx_files::workspace_path!("credits/themes.json"), "credits/themes.rs");
    generate_translators("credits/translators.rs")
}

fn generate_from_json(input_file: &str, output_file: &str) {
    build_script::cargo_rerun_if_changed(input_file);

    let json: HashMap<String, String> = gdsfx_files::read_json_file(input_file).unwrap();

    let names = json.keys();
    let links = json.values().map(|link| quote! { Some(#link) });

    gdsfx_build::write_output_rust(output_file, create_mapping_fn(names, links));
}

fn generate_translators(output_file: &str) {
    #[derive(Deserialize)]
    struct Locale {
        #[serde(rename = "language.translators")]
        translators: Vec<String>,
    }

    let mut locales = Vec::new();
    let mut translators = Vec::new();

    for file in gdsfx_files::read_dir(crate::i18n::LOCALES_DIR).unwrap() {
        let path = file.path();
        let Some(locale) = path.file_stem().and_then(|path| path.to_str()) else { continue };

        let locale_json: Locale = gdsfx_files::read_json_file(&path).unwrap();
        let locale_translators = locale_json.translators;

        locales.push(locale.to_string());
        translators.push(quote!(&[ #(#locale_translators),* ]));
    }

    gdsfx_build::write_output_rust(output_file, create_mapping_fn(locales, translators));
}

fn create_mapping_fn(keys: impl IntoIterator<Item = impl ToTokens>, values: impl IntoIterator<Item = impl ToTokens>) -> TokenStream {
    let keys = keys.into_iter();
    let values = values.into_iter();
    quote! {
        |key: &str| {
            match key {
                #(#keys => #values,)*
                _ => Default::default()
            }
        }
    }
}
