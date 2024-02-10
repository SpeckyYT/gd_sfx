use eframe::egui::Ui;
use gdsfx_library::{MusicLibrary, SfxLibrary};
use pretty_bytes::converter::convert as pretty_bytes;
use pretty_duration::pretty_duration;

use crate::{layout, backend::{AppState, LibraryPage}};

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    macro_rules! matcher {
        ($sfx:expr, $music:expr) => {
            match app_state.library_page {
                LibraryPage::Sfx => $sfx,
                LibraryPage::Music => $music,
            }
        };
    }

    ui.heading(t!(match app_state.library_page {
        LibraryPage::Sfx => "stats.library.sfx",
        LibraryPage::Music => "stats.library.music",
    }));

    ui.add_space(10.0);

    ui.label(t!(
        "stats.library.files",
        files = matcher!(sfx_library.sound_ids().len(), music_library.songs.len())
    ));

    let total_bytes = pretty_bytes(matcher!(sfx_library.total_bytes() as f64, music_library.total_bytes() as f64));
    ui.label(t!("stats.library.size", size = total_bytes));

    let total_duration = pretty_duration(&matcher!(sfx_library.total_duration(), music_library.total_duration()), None);
    ui.label(t!("stats.library.duration", duration = total_duration));

    ui.add_space(5.0);

    ui.label(t!("stats.library.version", version = matcher!(sfx_library.get_version().to_string(), music_library.version.to_string())));

    ui.add_space(20.0);

    ui.heading(t!(match app_state.library_page {
        LibraryPage::Sfx => "stats.files.sfx",
        LibraryPage::Music => "stats.files.music",
    }));
    
    ui.add_space(10.0);

    ui.label(t!("stats.files.downloaded", files = matcher!(app_state.get_sfx_count(), app_state.get_songs_count())));

    if let LibraryPage::Sfx = app_state.library_page {
        ui.label(t!("stats.sounds.unlisted", sounds = app_state.unlisted_sfx.len()));
    }
}
