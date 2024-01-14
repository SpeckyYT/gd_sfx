use eframe::egui::{Context, CentralPanel, Button};
use gdsfx_library::EntryKind;

use crate::{library_manager::LibraryManager, app_state::AppState};

pub fn render(ctx: &Context, app_state: &mut AppState, library_manager: &LibraryManager) {
    if let Some(entry) = app_state.selected_sfx.clone() {
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

                let file_handler = entry.create_file_handler(&app_state.settings.gd_folder);
                let file_exists = file_handler.file_exists();
    
                let download_button = Button::new(t!("sound.button.download"));
                if ui.add_enabled(!file_exists, download_button).clicked() {
                    library_manager.download_sound(&entry, app_state);
                }
    
                let delete_button = Button::new(t!("sound.button.delete"));
                if ui.add_enabled(file_exists, delete_button).clicked() {
                    file_handler.try_delete_file();
                }

                ui.add_space(10.0);
    
                if ui.button(t!("sound.button.play")).clicked() {
                    library_manager.play_sound(&entry, app_state);
                }
    
                let stop_button = Button::new(t!("sound.button.stop"));
                if ui.add_enabled(gdsfx_audio::is_playing_audio(), stop_button).clicked() {
                    gdsfx_audio::stop_all();
                }

                ui.add_space(10.0);

                let favorite_button_label = match app_state.favorites.has_favorite(entry.id) {
                    false => t!("sound.button.favorite.add"),
                    true => t!("sound.button.favorite.remove"),
                };
                if ui.button(favorite_button_label).clicked() {
                    app_state.favorites.toggle_favorite(entry.id);
                    ui.close_menu();
                }
            });
        }
    }
}
