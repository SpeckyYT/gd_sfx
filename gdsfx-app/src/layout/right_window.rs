use std::time::Duration;

use eframe::{egui::*, epaint::Color32};
use gdsfx_audio::AudioSettings;
use gdsfx_library::{BytesSize, EntryId};
use crate::images;

use crate::backend::{AppState, LibraryPage};

// TODO can we make this less of a list of ui elements
// and instead maybe put some stuff on the right side of the screen
// also make sure everything fits on the ui
pub fn render(ctx: &Context, app_state: &mut AppState) {
    CentralPanel::default().show(ctx, |ui| {
        match app_state.library_page {
            LibraryPage::Sfx => render_sfx_window(ui, app_state),
            LibraryPage::Music => render_music_window(ui, app_state),
        }
    });
}

fn render_sfx_window(ui: &mut Ui, app_state: &mut AppState) {
    let Some(entry) = &app_state.selected_sfx else { return };

    let entry_id = entry.id;

    ui.heading(&entry.name);

    ui.add_space(10.0);

    ui.code(entry.to_string());

    ui.add_space(10.0);

    render_sound_info(
        ui,
        entry.id,
        entry.bytes().unwrap_or(0),
        entry.duration().unwrap_or(Duration::ZERO),
        false,
    );

    ui.add_space(25.0);

    render_buttons(
        ui,
        app_state,
        app_state.is_sfx_downloaded(entry_id),
        app_state.favorites.has_favorite(entry_id),
        |app_state| app_state.play_sfx(entry_id),
        |app_state| app_state.download_sfx(entry_id),
        |app_state| app_state.delete_sfx(entry_id),
        |app_state| app_state.favorites.toggle_favorite(entry_id),
    );

    ui.add_space(10.0);

    render_audio_settings(ui, app_state);
}

fn render_sound_info(ui: &mut Ui, id: EntryId, bytes: BytesSize, duration: Duration, round_seconds: bool) {
    ui.heading(t!("sound.info.id", id = id));

    if bytes > 0 {
        ui.heading(t!("sound.info.size", size = pretty_bytes::converter::convert(bytes as f64)));
    }

    let duration = duration.as_secs_f32();
    if duration > 0.0 {
        ui.heading(t!(
            "sound.info.duration",
            duration = if round_seconds {
                format!("{duration:.0}s")
            } else {
                format!("{duration:.2}s")
            }
        ));
    }
}

const IMAGE_BUTTON_SIZE: Vec2 = Vec2::new(32.0, 32.0);

macro_rules! image_button {
    (
        $ui:expr,
        $source:expr $(=> rgb($r:expr, $g:expr, $b:expr))?,
        $size:expr,
        $enabled:expr $(,)?
    ) => {
        {
            let image: Image<'static> = $source.into();
            let image = image.tint(Color32::GRAY)
                $( .tint(Color32::from_rgb($r,$g,$b)) )?;
            let download_button = Button::image(image.fit_to_exact_size($size * 0.75)).min_size($size);
            $ui.add_enabled($enabled, download_button)
        }
    };
}

fn render_music_window(ui: &mut Ui, app_state: &mut AppState) {
    let Some(song) = &app_state.selected_music else { return };

    let song_id = song.id;

    ui.heading(&song.name);

    ui.add_space(10.0);

    ui.code(song.to_string());

    ui.add_space(10.0);

    render_sound_info(ui, song.id, song.bytes, song.duration, true);

    ui.add_space(25.0);

    render_buttons(
        ui,
        app_state,
        false,
        app_state.favorites.has_favorite(song_id),
        |app_state| (),
        |app_state| (),
        |app_state| (),
        |app_state| app_state.favorites.toggle_favorite(song_id),
    );

    ui.add_space(10.0);

    render_audio_settings(ui, app_state);
}

fn render_buttons<A,B,C,D>(
    ui: &mut Ui,
    app_state: &mut AppState,
    exists: bool,
    is_favorite: bool,
    play_cb: impl FnOnce(&mut AppState) -> A,
    download_cb: impl FnOnce(&mut AppState) -> B,
    delete_cb: impl FnOnce(&mut AppState) -> C,
    toggle_favorite_cb: impl FnOnce(&mut AppState) -> D,
) {
    ui.horizontal(|ui| {
        if image_button!(
            ui,
            images::PLAY,
            IMAGE_BUTTON_SIZE,
            true,
        ).clicked() {
            play_cb(app_state);
        }

        if image_button!(
            ui,
            images::STOP,
            IMAGE_BUTTON_SIZE,
            gdsfx_audio::is_playing_audio(),
        ).clicked() {
            gdsfx_audio::stop_all();
        }
    });

    ui.add_space(5.0);

    if app_state.is_gd_folder_valid() {
        ui.horizontal(|ui| {
            if image_button!(
                ui,
                images::DOWNLOAD,
                IMAGE_BUTTON_SIZE,
                !exists,
            ).clicked() {
                download_cb(app_state);
            }

            if image_button!(
                ui,
                images::TRASH,
                IMAGE_BUTTON_SIZE,
                exists,
            ).clicked() {
                delete_cb(app_state);
            }
        });
    } else {
        ui.colored_label(Color32::KHAKI, t!("settings.gd_folder.not_found"));
    }
    
    ui.add_space(5.0);

    let favorite_button_label = match is_favorite {
        true => images::STAR_SOLID,
        false => images::STAR_REGULAR,
    };
    if image_button!(
        ui,
        favorite_button_label,
        IMAGE_BUTTON_SIZE,
        true,
    ).clicked() {
        toggle_favorite_cb(app_state);
        ui.close_menu();
    }
}

fn render_audio_settings(ui: &mut Ui, app_state: &mut AppState) {
    ui.label(t!("sound.speed"));
    ui.add(Slider::new(&mut app_state.audio_settings.speed, -12.0..=12.0));
    
    ui.label(t!("sound.pitch"));
    ui.add(Slider::new(&mut app_state.audio_settings.pitch, -12.0..=12.0));

    ui.label(t!("sound.volume"));
    ui.add(Slider::new(&mut app_state.audio_settings.volume, 0.0..=2.0));

    ui.add_space(10.0);

    let reset_button = Button::new(t!("sound.reset"));
    let default_audio_settings = AudioSettings::default();
    if ui.add_enabled(app_state.audio_settings != default_audio_settings, reset_button).clicked() {
        app_state.audio_settings = default_audio_settings;
    }
}
