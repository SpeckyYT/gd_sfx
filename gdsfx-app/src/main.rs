use gdsfx_data::paths;

use crate::tabs::Tab;

pub use crate::gdsfx::GdSfx;

mod gdsfx;

mod layout;
mod tabs;

mod settings;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see gdsfx-app/build/i18n
gdsfx_build::include!("i18n.rs");

fn main() -> eframe::Result<()> {
    hide_console_window();
    GdSfx::run()
}

fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}
