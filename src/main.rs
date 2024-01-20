fn main() {
    hide_console_window();
    gdsfx_app::GdSfx::run().unwrap();
}

fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}
