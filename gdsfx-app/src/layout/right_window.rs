use eframe::{egui::*, epaint::Color32};
use gdsfx_audio::AudioSettings;
use gdsfx_library::{EntryKind, LibraryEntry, EntryId};

use crate::backend::AppState;

// TODO can we make this less of a list of ui elements
// and instead maybe put some stuff on the right side of the screen
// also make sure everything fits on the ui
pub fn render(ctx: &Context, app_state: &mut AppState) {
    CentralPanel::default().show(ctx, |ui| {
        let Some(entry) = &app_state.selected_sfx else { return };

        ui.heading(&entry.name);

        ui.add_space(10.0);

        ui.code(entry.to_string());

        ui.add_space(10.0);

        render_sound_info(ui, entry);

        ui.add_space(25.0);

        render_buttons(ui, app_state, entry.id);

        ui.add_space(10.0);

        render_audio_settings(ui, app_state);
    });
}

fn render_sound_info(ui: &mut Ui, entry: &LibraryEntry) {
    if let EntryKind::Sound { bytes, duration } = &entry.kind {
        ui.heading(t!("sound.info.id", id = entry.id));

        if *bytes > 0 {
            ui.heading(t!("sound.info.size", size = pretty_bytes::converter::convert(*bytes as f64)));
        }
    
        let duration = duration.as_secs_f32();
        if duration > 0.0 {
            ui.heading(t!("sound.info.duration", duration = format!("{duration:.2}s")));
        }
    }
}

fn render_buttons(ui: &mut Ui, app_state: &mut AppState, id: EntryId) {
    if app_state.is_gd_folder_valid() {
        let file_exists = app_state.is_sfx_downloaded(id);

        let download_button = Button::new(t!("sound.download"));
        if ui.add_enabled(!file_exists, download_button).clicked() {
            app_state.download_sfx(id);
        }

        let delete_button = Button::new(t!("sound.delete"));
        if ui.add_enabled(file_exists, delete_button).clicked() {
            app_state.delete_sfx(id);
        }
    } else {
        ui.colored_label(Color32::KHAKI, t!("settings.gd_folder.not_found"));
    }
    
    ui.add_space(10.0);

    if ui.button(t!("sound.play")).clicked() {
        app_state.play_sfx(id);
    }

    let stop_button = Button::new(t!("sound.stop"));
    if ui.add_enabled(app_state.audio_system.read().is_playing(), stop_button).clicked() {
        let _ = app_state.audio_system.write().stop_audio();
    }

    ui.add_space(10.0);

    let favorite_button_label = match app_state.favorites.has_favorite(id) {
        false => t!("sound.favorite.add"),
        true => t!("sound.favorite.remove"),
    };
    if ui.button(favorite_button_label).clicked() {
        app_state.favorites.toggle_favorite(id);
        ui.close_menu();
    }
}

fn render_audio_settings(ui: &mut Ui, app_state: &mut AppState) {
    let mut audio_system = app_state.audio_system.write();
    let audio_settings = &mut audio_system.settings;

    ui.add(Slider::new(&mut audio_settings.speed, -12..=12).text(t!("sound.speed")));
    ui.add(Slider::new(&mut audio_settings.pitch, -12..=12).text(t!("sound.pitch")));
    ui.add(Slider::new(&mut audio_settings.volume, 0.0..=2.0).text(t!("sound.volume")));
    ui.checkbox(&mut audio_settings.looping, t!("sound.loop"));
    ui.add(Slider::new(&mut audio_settings.start, 0..=1000).text(t!("sound.start")).clamp_to_range(false)); // TODO limit slider to sound length in ms
    ui.add(Slider::new(&mut audio_settings.end, 0..=1000).text(t!("sound.end")).clamp_to_range(false)); // TODO limit slider to sound length in ms
    ui.add(Slider::new(&mut audio_settings.fade_in, 0..=1000).text(t!("sound.fade_in")).clamp_to_range(false));
    ui.add(Slider::new(&mut audio_settings.fade_out, 0..=1000).text(t!("sound.fade_out")).clamp_to_range(false));

    ui.add_space(10.0);

    let reset_button = Button::new(t!("sound.reset"));
    let default_audio_settings = AudioSettings::default();
    if ui.add_enabled(*audio_settings != default_audio_settings, reset_button).clicked() {
        *audio_settings = default_audio_settings;
    }
}
