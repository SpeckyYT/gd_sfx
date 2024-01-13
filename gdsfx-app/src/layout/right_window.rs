use eframe::egui::{Context, CentralPanel, Button};
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::GdSfx;

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    if let Some(LibraryEntry {
        id, name, parent_id,
        kind: EntryKind::Sound { bytes, duration }
    }) = &gdsfx.selected_sfx
    {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(name);

            ui.add_space(25.0);

            // ui.code(sfx.get_string());

            ui.add_space(25.0);

            ui.heading(t!("sound.info.id", id = id));
            ui.heading(t!("sound.info.category.id", id = parent_id));
            ui.heading(t!("sound.info.size", size = pretty_bytes::converter::convert(*bytes as f64)));
            ui.heading(t!("sound.info.duration", duration = duration));

            ui.add_space(25.0);

            // let download_button = Button::new(t!("sound.button.download"));
            // if ui.add_enabled(!sfx.exists(), download_button).clicked() {
            //     sfx.download_and_store();
            // }
            
            // let delete_button = Button::new(t!("sound.button.delete"));
            // if ui.add_enabled(sfx.exists(), delete_button).clicked() {
            //     sfx.delete();
            // }

            // if ui.button(t!("sound.button.play")).clicked() { gdsfx_audio::play_sound(sfx); }
            if ui.button(t!("sound.button.stop")).clicked() { gdsfx_audio::stop_all(); }
        });
    }
}
