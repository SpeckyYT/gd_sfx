use std::env;
use std::path::Path;

use anyhow::Context;
pub use proc_macro2::TokenStream;
pub use build_script::cargo_rerun_if_changed;

pub const ICON_WIDTH: u32 = 256;
pub const ICON_HEIGHT: u32 = 256;

#[macro_export]
macro_rules! get_output {
    ( $macro:ident!($file:expr) ) => {
        $macro!(concat!(env!("OUT_DIR"), "/", $file));
    }
}

pub fn write_output_rust(path: impl AsRef<Path>, tokens: TokenStream) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    
    // write unformatted token stream first...
    gdsfx_data::write_file(&path, tokens.to_string()).unwrap();

    // ...so that it can be inspected if parsing the token stream fails...
    let parsed = syn::parse2(tokens)
        .with_context(|| format!("Couldn't parse token stream of {}", path.display()))
        .unwrap();
    
    // ...before finally writing the formatted version to the file
    gdsfx_data::write_file(&path, prettyplease::unparse(&parsed)).unwrap();
}

pub fn write_output_bytes(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    gdsfx_data::write_file(path, bytes).unwrap();
}
