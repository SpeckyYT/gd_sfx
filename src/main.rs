use eframe::NativeOptions;
use util::hide_console_window;

mod requests;
mod encoding;
mod library;
mod gui;
mod util;
mod audio;

fn main() {
    hide_console_window();

    let mut gdsfx = gui::GdSfx::default();

    gdsfx.get_cdn_url(false);
    gdsfx.get_sfx_version(false);
    gdsfx.get_sfx_library(false);

    gdsfx.run(NativeOptions {
        ..Default::default()
    });
}
