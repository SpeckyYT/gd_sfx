use eframe::egui::Ui;
use gdsfx_library::Library;
use pretty_bytes::converter::convert as pretty_bytes;

use crate::backend::AppState;

pub fn render(ui: &mut Ui, app_state: &AppState, library: &Library) {
    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    ui.label(t!("stats.library.files", files = library.total_sounds()));

    let total_bytes = pretty_bytes(library.total_bytes() as f64);
    ui.label(t!("stats.library.size", size = total_bytes));

    let total_duration = format!("{:.2}s", library.total_duration().as_secs_f32());
    ui.label(t!("stats.library.duration", duration = total_duration));

    ui.add_space(5.0);

    ui.label(t!("stats.library.version", version = library.get_version()));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    ui.label(t!("stats.files.downloaded", files = app_state.get_sfx_count()));
}
