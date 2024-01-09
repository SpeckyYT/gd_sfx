use crate::util::{geometry_dash_dir, self};
use quote::quote;

const OUTPUT_FILE: &str = "sfx_list.rs";

pub fn build() {
    let gd_folder = geometry_dash_dir();

    let ids = gd_folder.read_dir().unwrap()
    .map(|path| path.unwrap().path())
    .filter_map(|path| {
        let name = path.file_name().unwrap().to_str().unwrap();

        if name.starts_with("s") && name.ends_with(".ogg") {
            Some(name[1..name.len()-4].parse::<u32>().unwrap())
        } else {
            None
        }
    })
    .map(|id| quote!(#id))
    .collect::<Vec<_>>();

    let len = ids.len();

    let tokens = quote! {
        pub const ALL_SFX_IDS: [u32; #len] = [
            #(#ids),*
        ];
    };

    util::write_output_file(OUTPUT_FILE, tokens);
}


