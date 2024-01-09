pub mod gui;

// the build script generates the implementations of the functions below at compile-time
// → see build/credits.rs
include!(concat!(env!("OUT_DIR"), "/credits.rs"));

fn get_link(name: &str) -> Option<&str> {
    __get_link(name)
}

fn get_translators(locale: &str) -> &[&str] {
    __get_translators(locale)
}
