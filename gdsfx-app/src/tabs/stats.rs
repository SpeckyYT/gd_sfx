use eframe::egui::Ui;

use crate::library_manager::LibraryManager;
use pretty_bytes::converter::convert as pretty_bytes;

pub fn render(ui: &mut Ui, library_manager: &LibraryManager) {
    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    let total_files = library_manager.library.get_total_entries();
    ui.label(t!("stats.library.files", files = total_files));

    let total_bytes = pretty_bytes(library_manager.library.get_total_bytes() as f64);
    ui.label(t!("stats.library.size", size = total_bytes));

    let total_duration = library_manager.library.get_total_duration();
    ui.label(t!("stats.library.duration", duration = total_duration));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    // ui.label(t!("stats.files.downloaded", files = EXISTING_SOUND_FILES.lock().len()));
}
