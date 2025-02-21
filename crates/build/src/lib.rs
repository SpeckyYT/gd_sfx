use std::{env, path::Path};

pub use build_script::*;
pub use proc_macro2::TokenStream;

#[macro_export]
macro_rules! get_output {
    ( $macro:ident!($file:literal) ) => {
        $macro! { concat!(env!("OUT_DIR"), "/", $file) }
    }
}

pub fn write_output_rust(path: impl AsRef<Path>, tokens: TokenStream) {
    write_output_bytes(path, tokens.to_string());

    // commented out because this won't work with files that aren't valid rust files by themselves
    /*
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    gdsfx_files::create_parent_dirs(&path).unwrap();

    // write unformatted token stream first...
    gdsfx_files::write_file(&path, tokens.to_string()).unwrap();

    // ...so that it can be inspected if parsing the token stream fails...
    let parsed = syn::parse2(tokens)
    .with_context(|| format!("Couldn't parse token stream of {}", path.display()))
    .unwrap();

    // ...before finally writing the formatted version to the file
    gdsfx_files::write_file(&path, prettyplease::unparse(&parsed)).unwrap();
    */
}

pub fn write_output_bytes(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(path);
    files::create_parent_dirs(&path).unwrap();
    files::write_file(path, bytes).unwrap();
}
