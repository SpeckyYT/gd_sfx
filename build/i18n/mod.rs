use std::fs;

use crate::util;

const INCLUDE_STR: &str = include_str!("_include.rs");
const OUTPUT_PATH: &str = "i18n.rs";

pub fn build() {
    // write i18n!(...) macro invocation to OUTPUT_PATH to include!() it in main.rs
    // so that it reruns every time main.rs is compiled
    fs::write(util::get_output_file(OUTPUT_PATH), INCLUDE_STR)
        .unwrap_or_else(|e| panic!("Couldn't write to file '{OUTPUT_PATH}': {e}"));
}
