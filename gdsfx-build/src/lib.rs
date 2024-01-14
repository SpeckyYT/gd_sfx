use std::{env, fs};
use std::path::Path;

use anyhow::Context;
pub use proc_macro2::TokenStream;
pub use build_script::cargo_rerun_if_changed;

#[macro_export]
macro_rules! include {
    ( $file:expr ) => {
        include!(concat!(env!("OUT_DIR"), "/", $file));
    }
}

pub fn write_output_file(path: impl AsRef<Path>, tokens: TokenStream) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    
    // write unformatted token stream first...
    fs::write(&path, tokens.to_string())
        .with_context(|| format!("Couldn't write to file {}", path.display()))
        .unwrap();

    // ...so that it can be inspected if parsing the token stream fails...
    let parsed = syn::parse2(tokens)
        .with_context(|| format!("Couldn't parse token stream of {}", path.display()))
        .unwrap();
    
    // ...before finally writing the formatted version to the file
    fs::write(&path, prettyplease::unparse(&parsed))
        .with_context(|| format!("Couldn't write to file {}", path.display()))
        .unwrap();
}

pub fn write_output_bytes(path: impl AsRef<Path>, bytes: Vec<u8>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);

    fs::write(&path, bytes).unwrap();
}
