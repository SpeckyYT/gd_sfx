use std::collections::VecDeque;
use std::mem::size_of_val;
use std::sync::Arc;
use eframe::epaint::mutex::Mutex;
use eframe::egui::{self, TopBottomPanel};
use gdsfx_library::{MusicLibrary, SfxLibrary};
use memory_stats::memory_stats;
use once_cell::sync::Lazy;
use pretty_bytes::converter::convert as pretty_bytes;
use crate::backend::konami::KonamiString;
use crate::backend::AppState;

const MIN_FPS_HISTORY_SIZE: usize = 20;
const MAX_FPS_HISTORY_TIME: f64 = 2.0;

#[derive(Default)]
pub struct DebugMode {
    fps_history: VecDeque<(f64,f64)>,
}

pub static DEBUG_MODE: Lazy<Arc<Mutex<Option<DebugMode>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

fn toggle_debug_mode() {
    let mut debug_mode = DEBUG_MODE.lock();
    *debug_mode = debug_mode.take().xor(Some(Default::default()));
}

const DEBUG_KONAMI: KonamiString = {
    use super::Key::*;
    KonamiString::new(
        &[
            ArrowUp, ArrowUp,
            ArrowDown, ArrowDown,
            ArrowLeft, ArrowRight,
            ArrowLeft, ArrowRight,
            B, A,
        ],
        &toggle_debug_mode,
    )
};

pub fn debug_display(ctx: &egui::Context, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    app_state.konami.push(DEBUG_KONAMI);

    if let Some(ref mut debug_mode) = *DEBUG_MODE.lock() {
        TopBottomPanel::bottom("debug_panel")
            .show(ctx, |ui| {
                ui.heading(t!("debug.mode"));

                let history = &mut debug_mode.fps_history;

                let current_time = ui.input(|i| i.time);
                let (last_time, _) = *history.iter().last().unwrap_or(&(0.0, 0.0));
                
                // this is so bad
                history.push_back((current_time, current_time - last_time));

                loop {
                    if current_time - history[0].0 > MAX_FPS_HISTORY_TIME && history.len() > MIN_FPS_HISTORY_SIZE {
                        history.pop_front();
                    } else {
                        break
                    }
                }

                let average = history.iter()
                    .map(|(_, i)| i)
                    .sum::<f64>() / history.len() as f64;

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
                ui.label(t!("debug.frame_time_size", size = history.len()));
            });
    }
}
