use std::collections::HashMap;

use gdsfx_data::paths;
use quote::quote;

type Credits = HashMap<String, String>;

const OUTPUT_FILE: &str = "credits.rs";

pub fn build() {
    gdsfx_build::cargo_rerun_if_changed(paths::build::CREDITS_FILE);

    let credits_json: Credits = gdsfx_data::read_json_file(paths::build::CREDITS_FILE).unwrap();

    let names = credits_json.keys();
    let links = credits_json.values();

    let tokens = quote! {
        fn __get_link(name: &str) -> Option<&str> {
            match name {
                #(#names => Some(#links),)*
                _ => None
            }
        }
    };

    gdsfx_build::write_output_file(OUTPUT_FILE, tokens);
}
