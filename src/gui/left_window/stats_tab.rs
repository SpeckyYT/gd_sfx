use eframe::egui::Ui;

use crate::{gui::GdSfx, util, library::LibraryEntry, stats::EXISTING_SOUND_FILES};

struct Stats {
    bytes: u128,
    duration: u128,
    files: i64,
}

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx) {
    fn get_sound_stats(entry: &LibraryEntry) -> Stats {
        match entry {
            LibraryEntry::Category { children, .. } => children
                .iter()
                .map(get_sound_stats)
                .reduce(|a, b| Stats {
                    bytes: a.bytes + b.bytes,
                    duration: a.duration + b.duration,
                    files: a.files + b.files
                })
                .unwrap_or(Stats { bytes: 0, duration: 0, files: 1 }),

            LibraryEntry::Sound { bytes, duration, .. } => Stats {
                bytes: *bytes as u128,
                duration: *duration as u128,
                files: 1
            }
        }
    }

    let library = &gdsfx.sfx_library.as_ref().unwrap().sound_effects;
    let Stats { bytes, duration, files } = get_sound_stats(library);

    ui.heading(t!("stats.library"));

    ui.add_space(10.0);

    ui.label(t!("stats.library.files", files = files));
    ui.label(t!("stats.library.size", size = pretty_bytes::converter::convert(bytes as f64)));
    ui.label(t!("stats.library.duration", duration = util::stringify_duration(duration as i64)));

    ui.add_space(20.0);

    ui.heading(t!("stats.files"));
    
    ui.add_space(10.0);

    ui.label(t!("stats.files.downloaded", files = EXISTING_SOUND_FILES.lock().unwrap().len()));
}