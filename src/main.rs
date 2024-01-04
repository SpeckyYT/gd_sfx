use eframe::{NativeOptions, egui::ViewportBuilder, epaint::Vec2};
use util::{hide_console_window, TOTAL_WIDTH, TOTAL_HEIGHT};

mod requests;
mod encoding;
mod library;
mod gui;
mod util;
mod audio;
mod favourites;

fn main() {
    hide_console_window();

    let mut gdsfx = gui::GdSfx::default();

    gdsfx.get_cdn_url(false);
    gdsfx.get_sfx_version(false);
    gdsfx.get_sfx_library(false);

    gdsfx.run(NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size(Vec2 {x: TOTAL_WIDTH, y: TOTAL_HEIGHT}),
        ..Default::default()
    });
}
