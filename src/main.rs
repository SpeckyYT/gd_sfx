use std::thread;

use eframe::{NativeOptions, egui::ViewportBuilder, epaint::Vec2};
use settings::FIRST_READ;
use stats::add_existing_sfx_files;
use util::{hide_console_window, TOTAL_WIDTH, TOTAL_HEIGHT};

#[macro_use]
extern crate rust_i18n;

mod requests;
mod encoding;
mod library;
mod gui;
mod util;
mod audio;
mod settings;
mod stats;
mod tools;

#[cfg(test)]
mod test;

// build.rs reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file
include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

fn main() {
    hide_console_window();

    thread::spawn(add_existing_sfx_files);

    let mut gdsfx = gui::GdSfx::default();

    gdsfx.get_cdn_url(false);
    gdsfx.get_sfx_version(false);
    gdsfx.get_sfx_library(false);

    lazy_static::initialize(&FIRST_READ);

    gdsfx.run(NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2 { x: TOTAL_WIDTH, y: TOTAL_HEIGHT })
            .with_min_inner_size(Vec2 { x: TOTAL_WIDTH * 0.7, y: TOTAL_HEIGHT * 0.7 }),

        follow_system_theme: false,
        ..Default::default()
    });
}
