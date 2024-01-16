use std::{sync::Arc, thread};

use eframe::epaint::mutex::Mutex;
use gdsfx_library::Library;

use super::AppState;

type Stats = Arc<Mutex<Option<(u128, u128)>>>;

pub fn download_all(app_state: &AppState, library: &Library, stats: Stats) {
    let app_state = app_state.clone();
    let library = library.clone();

    thread::spawn(move || {
        *stats.lock() = Some((0, library.get_total_entries() as u128));

        for sound in library.get_all_sounds() {
            app_state.download_sound(sound);
            if let Some(ref mut stats) = *stats.lock() {
                *stats = (stats.0 + 1, stats.1);
            }
        }

        *stats.lock() = None;
    });
}

pub fn download_from_range() {

}
