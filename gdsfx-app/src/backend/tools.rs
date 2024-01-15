use std::{sync::Arc, thread::spawn};

use eframe::epaint::mutex::Mutex;
use gdsfx_library::{LibraryEntry, EntryKind};
use crate::Library;

use super::AppState;

type STATS = Arc<Mutex<Option<(u128, u128)>>>;

pub fn download_all(library: Library, app_state: &AppState, stats: STATS) {
    let root = library.lock().get_root();

    *stats.lock() = Some((0, library.lock().get_total_entries() as u128));

    fn recursive<'a>(library: Library, entries: impl Iterator<Item = &'a LibraryEntry>, app_state: &AppState, stats: STATS) {
        entries.for_each(|entry| {
            match entry.kind {
                EntryKind::Category =>
                    recursive(
                        library.clone(),
                        library.lock().get_children(entry),
                        app_state,
                        stats.clone(),
                    ),
                EntryKind::Sound { .. } => {
                    app_state.download_sound(entry);
                    let mut lock = stats.lock();
                    let previous = *lock;
                    if let Some(previous) = previous {
                        *lock = Some((previous.0, previous.1 + 1));
                    }
                },
            };
        })
    }
    
    spawn(move || {
        recursive(
            library,
            library.lock().get_children(root),
            app_state,
            stats.clone(),
        );

        *stats.lock() = None;
    });
}
