use std::{sync::Arc, thread};

use eframe::epaint::mutex::Mutex;
use gdsfx_library::{Library, LibraryEntry, EntryKind};

use super::AppState;

type Stats = Arc<Mutex<Option<(u128, u128)>>>;

pub fn download_all(app_state: Arc<Mutex<AppState>>, library: Arc<Mutex<Library>>, stats: Stats) {
    *stats.lock() = Some((0, library.lock().get_total_entries() as u128));

    let library = library.clone();
    
    thread::spawn(move || {
        for sound in library.lock().get_all_sounds() {
            app_state.lock().download_sound(sound);
            if let Some(ref mut stats) = *stats.lock() {
                *stats = (stats.0, stats.1 + 1);
            }
        }

        *stats.lock() = None;
    });
}
