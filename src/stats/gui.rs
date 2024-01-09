use eframe::egui::Ui;

use crate::{gui::GdSfx, util, stats::{self, *}};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    let library = &gdsfx.sfx_library.as_ref().unwrap().sound_effects;
    let Stats { bytes, duration, files } = stats::get_sound_stats(library);

    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    ui.label(t!("stats.library.files", files = files));
    ui.label(t!("stats.library.size", size = pretty_bytes::converter::convert(bytes as f64)));
    ui.label(t!("stats.library.duration", duration = util::stringify_duration(duration as i64)));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    ui.label(t!("stats.files.downloaded", files = EXISTING_SOUND_FILES.lock().len()));
}
