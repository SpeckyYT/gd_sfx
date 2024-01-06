use eframe::egui::{Context, CentralPanel, Button};
use pretty_bytes::converter::convert;

use crate::{audio, util::stringify_duration};

use super::GdSfx;

pub fn render(gdsfx: &mut GdSfx, ctx: &Context) {
    if let Some(sfx) = &gdsfx.selected_sfx {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(sfx.name());

            ui.add_space(25.0);

            ui.code(sfx.get_string());

            ui.add_space(25.0);

            ui.heading(t!("sound.info.id", id = sfx.id()));
            ui.heading(t!("sound.info.category.id", id = sfx.parent()));
            ui.heading(t!("sound.info.size", size = convert(sfx.bytes() as f64)));
            ui.heading(t!("sound.info.duration", duration = stringify_duration(sfx.duration())));

            ui.add_space(25.0);

            let download_button = Button::new(t!("sound.button.download"));
            if ui.add_enabled(!sfx.exists(), download_button).clicked() {
                sfx.download_and_store();
            }
            
            let delete_button = Button::new(t!("sound.button.delete"));
            if ui.add_enabled(sfx.exists(), delete_button).clicked() {
                sfx.delete();
            }

            if ui.button(t!("sound.button.play")).clicked() { audio::play_sound(sfx); }
            if ui.button(t!("sound.button.stop")).clicked() { audio::stop_audio(); }
        });
    }
}
