use std::collections::VecDeque;
use std::mem::size_of_val;
use std::path::MAIN_SEPARATOR;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use eframe::epaint::mutex::Mutex;
use eframe::{egui::*, epaint::Color32};
use gdsfx_audio::AudioSettings;
use gdsfx_library::{BytesSize, EntryId, FileEntry, MusicFileEntry, MusicLibrary, SfxFileEntry, SfxLibrary};
use memory_stats::memory_stats;
use once_cell::sync::Lazy;
use pretty_bytes::converter::convert as pretty_bytes;

use crate::backend::konami::KonamiString;
use crate::images;
use crate::backend::{AppState, LibraryPage};

// TODO can we make this less of a list of ui elements
// and instead maybe put some stuff on the right side of the screen
// also make sure everything fits on the ui
pub fn render(ctx: &Context, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    CentralPanel::default().show(ctx, |ui| {
        match app_state.library_page {
            LibraryPage::Sfx => render_sfx_window(ui, app_state),
            LibraryPage::Music => render_music_window(ui, app_state),
        }
    });
    debug_display(ctx, app_state, sfx_library, music_library);
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

    render_buttons(ui, app_state, entry_id, SfxFileEntry::new(entry_id), app_state.is_sfx_downloaded(entry_id));

    ui.add_space(10.0);

    render_audio_settings(ui, app_state);
}

fn render_sound_info(ui: &mut Ui, id: EntryId, bytes: BytesSize, duration: Duration, round_seconds: bool) {
    ui.heading(t!("sound.info.id", id = id));

    if bytes > 0 {
        ui.heading(t!("sound.info.size", size = pretty_bytes(bytes as f64)));
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

    render_buttons(ui, app_state, song_id, MusicFileEntry::new(song_id), app_state.is_music_downloaded(song_id));

    ui.add_space(10.0);

    render_audio_settings(ui, app_state);
}

fn render_buttons(ui: &mut Ui, app_state: &mut AppState, id: EntryId, file_entry: impl FileEntry + 'static, is_downloaded: bool) {
    ui.horizontal(|ui| {
        if image_button!(
            ui,
            images::PLAY,
            IMAGE_BUTTON_SIZE,
            true,
        ).clicked() {
            app_state.play_sound(file_entry);
        }

        if image_button!(
            ui,
            images::STOP,
            IMAGE_BUTTON_SIZE,
            app_state.audio_system.read().is_playing(),
        ).clicked() {
            let _ = app_state.audio_system.write().stop_audio();
        }
    });

    ui.add_space(5.0);

    if app_state.is_gd_folder_valid() {
        ui.horizontal(|ui| {
            if image_button!(
                ui,
                images::DOWNLOAD,
                IMAGE_BUTTON_SIZE,
                !is_downloaded,
            ).clicked() {
                app_state.download_sound(file_entry);
            }

            if image_button!(
                ui,
                images::TRASH,
                IMAGE_BUTTON_SIZE,
                is_downloaded,
            ).clicked() {
                app_state.delete_sound(file_entry);
            }
        });
    } else {
        ui.colored_label(Color32::KHAKI, t!("settings.gd_folder.not_found"));
    }

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        if image_button!(
            ui,
            match app_state.favorites.has_favorite(id) {
                true => images::STAR_SOLID,
                false => images::STAR_REGULAR,
            },
            IMAGE_BUTTON_SIZE,
            true,
        ).clicked() {
            app_state.favorites.toggle_favorite(id);
            ui.close_menu();
        }

        if image_button!(
            ui,
            images::RIGHT_TO_BRACKET,
            IMAGE_BUTTON_SIZE,
            true,
        ).clicked() {
            let path = format!(
                "{}{MAIN_SEPARATOR}{}",
                &app_state.settings.gd_folder,
                &file_entry.get_file_name(),
            );

            #[cfg(target_os = "windows")]
            {
                let _ = Command::new("explorer")
                    .args([ "/select,", &path ])
                    .spawn();
            }

            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("open")
                .args(&[ "-R", &path ])
                .spawn();
            }

            #[cfg(target_os = "linux")]
            {
                let _ = Command::new("xdg-open")
                .args(&[ &path ])
                .spawn();
            }
        }
    });
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

const AVERAGE_SIZE: usize = 20;

static FPS_HISTORY: Lazy<Arc<Mutex<Option<VecDeque<(f64,f64)>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

fn toggle_fps_history() {
    let mut history = FPS_HISTORY.lock();
    *history = history.take().xor(Some(Default::default()));
}

const DEBUG_KONAMI: KonamiString = {
    use Key::*;
    KonamiString::new(
        &[
            ArrowUp, ArrowUp,
            ArrowDown, ArrowDown,
            ArrowLeft, ArrowRight,
            ArrowLeft, ArrowRight,
            B, A,
        ],
        &toggle_fps_history,
    )
};

fn debug_display(ctx: &Context, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    app_state.konami.push(DEBUG_KONAMI);

    if let Some(ref mut history) = *FPS_HISTORY.lock() {
        TopBottomPanel::bottom("debug_panel")
            .show(ctx, |ui| {
                ui.heading(t!("debug.mode"));

                let current_time = ui.input(|i| i.time);
                let (last_time, _) = *history.iter().last().unwrap_or(&(0.0, 0.0));
                
                // this is so bad
                history.push_back((current_time, current_time - last_time));
                while history.len() > AVERAGE_SIZE {
                    history.pop_front();
                }

                let average_size = AVERAGE_SIZE.min(history.len());

                let average = history.iter()
                    .rev()
                    .take(average_size)
                    .map(|(_, i)| i)
                    .sum::<f64>() / average_size as f64;

                ui.label(t!(
                    "debug.build_kind",
                    kind = if cfg!(debug_assertions) {
                        t!("debug.build_kind.debug")
                    } else {
                        t!("debug.build_kind.release")
                    }
                ));
                if let Some(memory_stats) = memory_stats() {
                    ui.label(t!("debug.memory.physical", bytes = pretty_bytes(memory_stats.physical_mem as f64)));
                    ui.label(t!("debug.memory.virtual", bytes = pretty_bytes(memory_stats.virtual_mem as f64)));
                }
                ui.label(t!("debug.memory.app_state", bytes = pretty_bytes(size_of_val(app_state) as f64)));
                ui.label(t!("debug.memory.sfx_library", bytes = pretty_bytes(size_of_val(sfx_library) as f64)));
                ui.label(t!("debug.memory.music_library", bytes = pretty_bytes(size_of_val(music_library) as f64)));
                ui.label(t!("debug.average_frame_time", ms = format!("{:.2}", average * 1000.0)));
                ui.label(t!("debug.average_fps", fps = format!("{:.2}", 1.0 / average)));
            });
    }
}
