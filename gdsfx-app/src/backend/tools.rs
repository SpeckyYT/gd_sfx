use std::{sync::Arc, thread, ops::RangeInclusive, fs};

use eframe::epaint::mutex::Mutex;
use gdsfx_library::{Library, LibraryEntry, EntryKind, stats::Centiseconds};
use rayon::prelude::*;

use super::AppState;

type Stats = Arc<Mutex<Option<(u128, u128)>>>;

pub fn download_all(app_state: &AppState, library: &Library, stats: Stats) {
    let app_state = app_state.clone();
    let library = library.clone();

    *stats.lock() = Some((0, library.total_entries() as u128));

    thread::spawn(move || {
        library.iter_sounds()
            .collect::<Vec<_>>()
            .into_par_iter()
            .for_each(|sound| {
                app_state.download_sound(sound);
                if let Some(ref mut stats) = *stats.lock() {
                    *stats = (stats.0 + 1, stats.1);
                }
            });

        *stats.lock() = None;
    });
}

pub fn download_from_range(app_state: &AppState, stats: Stats, range: RangeInclusive<u32>) {
    let app_state = app_state.clone();

    *stats.lock() = Some((0, range.clone().count() as u128));

    thread::spawn(move || {
        range.into_par_iter()
        .for_each(|id| {
            app_state.download_sound(&LibraryEntry {
                id,
                name: String::new(),
                parent_id: 0,
                kind: EntryKind::Sound { bytes: 0, duration: Centiseconds(0) },
            })
            .map(|handle| handle.join());

            if let Some(ref mut stats) = *stats.lock() {
                *stats = (stats.0 + 1, stats.1);
            }
        });

        *stats.lock() = None;
    });
}

pub fn delete_all_sfx(app_state: &AppState, stats: Stats) {
    if let Ok(read_dir) = fs::read_dir(&app_state.settings.gd_folder) {
        let read_dir = read_dir.collect::<Vec<_>>();
        let count = read_dir.len();

        *stats.lock() = Some((0, count as u128));

        thread::spawn(move || {
            read_dir.into_par_iter()
            .for_each(|entry| {
                if let Ok(entry) = entry {
                    let is_valid = entry.file_name()
                        .to_str()
                        .filter(|s| s.starts_with('s') && s.ends_with(".ogg"))
                        .map(|s| &s[1..s.len()-4])
                        .filter(|s| s.parse::<u32>().is_ok())
                        .is_some();

                    if is_valid {
                        let _ = fs::remove_file(entry.path());
                    }
                }
                 
                if let Some(ref mut stats) = *stats.lock() {
                    *stats = (stats.0 + 1, stats.1);
                }
            });

            *stats.lock() = None;
        });
    }
}
