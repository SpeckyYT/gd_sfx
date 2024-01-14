use eframe::egui::{Context, CentralPanel, Button};
use gdsfx_library::EntryKind;

use crate::GdSfx;

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    if let Some(entry) = gdsfx.selected_sfx.clone() {
        if let EntryKind::Sound { bytes, duration } = &entry.kind {
            CentralPanel::default().show(ctx, |ui| {
                ui.heading(&entry.name);
    
                ui.add_space(25.0);
    
                ui.code(entry.to_string());
    
                ui.add_space(25.0);
    
                ui.heading(t!("sound.info.id", id = entry.id));
                ui.heading(t!("sound.info.category.id", id = entry.parent_id));
                ui.heading(t!("sound.info.size", size = pretty_bytes::converter::convert(*bytes as f64)));
                ui.heading(t!("sound.info.duration", duration = duration));
    
                ui.add_space(25.0);
    
                let download_button = Button::new(t!("sound.button.download"));
                if ui.add_enabled(!entry.file_exists(), download_button).clicked() {
                    gdsfx.download_sound(&entry);
                }
    
                let delete_button = Button::new(t!("sound.button.delete"));
                if ui.add_enabled(entry.file_exists(), delete_button).clicked() {
                    entry.try_delete_file();
                }
    
                if ui.button(t!("sound.button.play")).clicked() {
                    gdsfx.play_sound(&entry);
                }
    
                if ui.button(t!("sound.button.stop")).clicked() {
                    gdsfx_audio::stop_all();
                }
            });
        }
    }
}
