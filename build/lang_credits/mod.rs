// im too tired for this

// use std::{fs, path::PathBuf};

// use crate::util;

// const INCLUDE_STR: &str = include_str!("_include.rs");
// const OUTPUT_PATH: &str = "credits.rs";

// const LANG_FILES_DIR: &str = "lang";

// pub fn build() {
//     let lang_files = PathBuf::from(LANG_FILES_DIR).read_dir()
//         .unwrap_or_else(|e| panic!("Couldn't read from directory '{LANG_FILES_DIR}': {e}"))
//         .flatten();

//     for file in lang_files {
//         let lang_code = file.path()
//             .file_stem().unwrap()
//             .to_str().unwrap();

//         let path = file.path()
//             .into_os_string()
//             .into_string().unwrap();

//         let json = util::read_json(&path);

//         for translator in json["language.translators"].as_array().unwrap() {
//             // translator["name"].as_str().unwrap()
//             // translator["link"].as_str().unwrap()
//         }
//     }

//     fs::write(util::get_output_file(OUTPUT_PATH), INCLUDE_STR)
//         .unwrap_or_else(|e| panic!("Couldn't write to file '{OUTPUT_PATH}': {e}"));
// }
