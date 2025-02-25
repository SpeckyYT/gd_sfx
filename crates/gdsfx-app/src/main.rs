use std::sync::Arc;

use eframe::egui::{IconData, ViewportBuilder};
use eframe::{HardwareAcceleration, NativeOptions};

use gdsfx_app::{layout, GdSfx};

fn hide_console_window() {
    #[cfg(windows)]
    unsafe { winapi::um::wincon::FreeConsole(); }
}

fn main() -> eframe::Result<()> {
    if !cfg!(debug_assertions) {
        hide_console_window();
    }

    // build script converts png icon into bytes
    const ICON_BYTES: &[u8] = gdsfx_build::get_output!(include_bytes!("icon.bin"));

    let icon = IconData {
        rgba: ICON_BYTES.to_vec(),
        width: gdsfx_shared::consts::ICON_SIZE,
        height: gdsfx_shared::consts::ICON_SIZE,
    };

    let viewport = ViewportBuilder {
        inner_size: Some(layout::DEFAULT_WINDOW_SIZE),
        min_inner_size: Some(layout::DEFAULT_WINDOW_SIZE * layout::MIN_SCALE_FACTOR),
        resizable: Some(true),
        icon: Some(Arc::new(icon)),

        ..Default::default()
    };

    let options = NativeOptions {
        viewport,
        hardware_acceleration: HardwareAcceleration::Preferred,
        vsync: false,

        ..Default::default()
    };

    eframe::run_native(
        gdsfx_shared::consts::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(GdSfx::new(cc))))
    )
}
