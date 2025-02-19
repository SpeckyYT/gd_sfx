use ahash::HashMap;
use gdsfx_files::paths;
use quote::{format_ident, quote};

const CREDITS_OUTPUT_FILE: &str = "credits.rs";
const THEME_OUTPUT_FILE: &str = "theme_credits.rs";

pub fn build() {
    println!("{CREDITS_OUTPUT_FILE}");
    generate_file(
        paths::build::CREDITS,
        CREDITS_OUTPUT_FILE,
        "get_link",
    );
    generate_file(
        paths::build::THEME_CREDITS,
        THEME_OUTPUT_FILE,
        "get_theme_credit",
    );
}

fn generate_file(input_file: &str, output_file: &str, function_name: &str) {
    build_script::cargo_rerun_if_changed(input_file);

    let json: HashMap<String, String> = gdsfx_files::read_json_file(input_file).unwrap();

    let names = json.keys();
    let links = json.values();

    let fn_name = format_ident!("{}", function_name);

    let tokens = quote! {
        fn #fn_name (name: &str) -> Option<&str> {
            match name {
                #(#names => Some(#links),)*
                _ => None
            }
        }
    };

    gdsfx_build::write_output_rust(output_file, tokens);
}
