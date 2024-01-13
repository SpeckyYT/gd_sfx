use gdsfx_build::TokenStream;
use gdsfx_data::paths;
use quote::quote;

pub fn build() -> TokenStream {
    let locales_dir = paths::build::LOCALES_DIR;

    quote! {
        #[macro_use]
        extern crate rust_i18n;

        // if the build script reruns, it forces this proc macro to rerun too 
        i18n!(#locales_dir, fallback = "en_US");
    }
}
