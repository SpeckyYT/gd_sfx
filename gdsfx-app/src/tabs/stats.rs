use std::time::Instant;

use eframe::egui::Ui;
use gdsfx_library::EntryKind;

use crate::GdSfx;
use pretty_bytes::converter::convert as pretty_bytes;

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    // let Stats { bytes, duration, files } = stats::get_sound_stats(library);

    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    ui.label(t!("stats.library.files", files = gdsfx.library.get_entries().len()));
    ui.label(t!("stats.library.size", size = pretty_bytes(gdsfx.library.get_total_bytes() as f64)));
    ui.label(t!("stats.library.duration", duration = gdsfx.library.get_total_duration()));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    // ui.label(t!("stats.files.downloaded", files = EXISTING_SOUND_FILES.lock().len()));
}
