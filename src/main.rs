use std::thread;

use eframe::{NativeOptions, egui::ViewportBuilder, epaint::Vec2};
use util::{TOTAL_WIDTH, TOTAL_HEIGHT};

mod audio;
mod gui;
mod util;

mod credits;
mod favorites;
mod library;
mod settings;
mod stats;
mod tools;

#[cfg(test)]
mod test;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see build/i18n.rs
include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

fn main() {
    util::hide_console_window();

    thread::spawn(stats::add_existing_sfx_files);

    let mut gdsfx = gui::GdSfx::default();

    gdsfx.get_cdn_url(false);
    gdsfx.get_sfx_version(false);
    gdsfx.get_sfx_library(false);

    library::update_unlisted_sfx(&gdsfx.sfx_library.as_ref().unwrap().sound_effects);

    // set default locale, will be overwritten by reading settings
    rust_i18n::set_locale("en_US");
    lazy_static::initialize(&settings::FIRST_READ);

    gdsfx.run(NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2 { x: TOTAL_WIDTH, y: TOTAL_HEIGHT })
            .with_min_inner_size(Vec2 { x: TOTAL_WIDTH * 0.7, y: TOTAL_HEIGHT * 0.7 }),

        follow_system_theme: false,
        ..Default::default()
    });
}
