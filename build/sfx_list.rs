use std::ffi::OsStr;

use quote::quote;

use crate::util;

const OUTPUT_FILE: &str = "sfx_list.rs";

pub fn build() {
    let gd_folder = util::geometry_dash_dir();

    let ids = gd_folder.read_dir().unwrap()
        .flatten()
        .map(|path| path.path())
        .filter_map(|path| {
            if let Some(name) = path.file_name().and_then(OsStr::to_str) {
                if name.starts_with('s') && name.ends_with(".ogg") {
                    return Some(name[1..name.len()-4].parse::<u32>().unwrap())
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let len = ids.len();

    let tokens = quote! {
        pub const ALL_SFX_IDS: [u32; #len] = [
            #(#ids),*
        ];
    };

    util::write_output_file(OUTPUT_FILE, tokens);
}
