use std::{thread, hint::black_box};

use eframe::{NativeOptions, egui::ViewportBuilder, epaint::Vec2, Theme};
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

#[cfg(test)]
mod test;

i18n!("lang", fallback = "en_US");

fn main() {
    hide_console_window();

    thread::spawn(add_existing_sfx_files);

    let mut gdsfx = gui::GdSfx::default();

    gdsfx.get_cdn_url(false);
    gdsfx.get_sfx_version(false);
    gdsfx.get_sfx_library(false);

    black_box(&FIRST_READ);

    gdsfx.run(NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size(Vec2 {x: TOTAL_WIDTH, y: TOTAL_HEIGHT}),

        follow_system_theme: false,
        default_theme: Theme::Dark,

        ..Default::default()
    });
}
