use ahash::HashMap;
use gdsfx_files::paths;
use quote::quote;

type Credits = HashMap<String, String>;

const OUTPUT_FILE: &str = "credits.rs";

pub fn build() {
    gdsfx_build::cargo_rerun_if_changed(paths::build::CREDITS_FILE);

    let credits_json: Credits = gdsfx_files::read_json_file(paths::build::CREDITS_FILE).unwrap();

    let names = credits_json.keys();
    let links = credits_json.values();

    let tokens = quote! {
        fn get_link(name: &str) -> Option<&str> {
            match name {
                #(#names => Some(#links),)*
                _ => None
            }
        }
    };

    gdsfx_build::write_output_rust(OUTPUT_FILE, tokens);
}
