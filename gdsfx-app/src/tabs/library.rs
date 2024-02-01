use std::time::Duration;

use eframe::egui::{CollapsingHeader, ComboBox, Ui};
use gdsfx_library::{EntryId, MusicLibrary, SfxLibrary};
use gdsfx_library::sfx::{SfxLibraryEntry, EntryKind};
use itertools::Itertools;

use crate::backend::search::MusicFilters;
use crate::backend::LibraryPage;
use crate::{layout, backend::{AppState, settings::SearchFilterMode}};

const UNLISTED_ID: EntryId = EntryId::MAX;

pub fn render(ui: &mut Ui, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);
    layout::add_search_area(ui, &mut app_state.search_settings);

    match app_state.library_page {
        LibraryPage::Sfx => render_sfx_library(ui, app_state, sfx_library),
        LibraryPage::Music => render_music_library(ui, app_state, music_library),
    }
}

fn render_sfx_library(ui: &mut Ui, app_state: &mut AppState, library: &SfxLibrary) {
    let collapse_all = ui.button(t!("library.collapse_all")).clicked();

    let categories: Vec<&SfxLibraryEntry> = library.iter_children(library.get_root()).collect();
    render_sfx_recursive(ui, app_state, library, categories, collapse_all);

    let mut unlisted_sounds: Vec<SfxLibraryEntry> = app_state.unlisted_sfx.iter()
        .map(|&id| SfxLibraryEntry {
            id,
            name: id.to_string(),
            parent_id: UNLISTED_ID,
            kind: EntryKind::Sound {
                bytes: 0,
                duration: Duration::ZERO,
            },
        })
        .filter(|entry| app_state.is_matching_entry(entry, library))
        .collect();

    let enabled = !unlisted_sounds.is_empty();
    if !enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide { return }

    unlisted_sounds.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));

    ui.separator();

    ui.add_enabled_ui(enabled, |ui| {
        let collapsing = CollapsingHeader::new(t!("library.unlisted_sfx"))
            .open((!enabled || collapse_all).then_some(false));

        let response = collapsing.show(ui, |ui| {
            for entry in unlisted_sounds {
                layout::add_sfx_button(ui, app_state, library, &entry);
            }
        });

        let text = t!("library.unlisted_sfx.hint", tool = t!("tools.download_from_range"));
        response.header_response.on_disabled_hover_text(text);
    });
}

fn render_sfx_recursive(ui: &mut Ui, app_state: &mut AppState, library: &SfxLibrary, mut entries: Vec<&SfxLibraryEntry>, collapse_all: bool) {
    entries.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(*a, *b));
    for entry in entries {
        match entry.kind {
            EntryKind::Category => {
                let is_enabled = app_state.is_matching_entry(entry, library);
    
                if !is_enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide {
                    continue // skip rendering entirely
                }
    
                ui.add_enabled_ui(is_enabled, |ui| {
                    let collapsing = CollapsingHeader::new(&entry.name)
                        .open((!is_enabled || collapse_all).then_some(false));
    
                    collapsing.show(ui, |ui| {
                        render_sfx_recursive(ui, app_state, library, library.iter_children(entry).collect(), collapse_all);
                    });
                });
            }
            EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library, entry),
        }
    }
}

fn render_music_library(ui: &mut Ui, app_state: &mut AppState, library: &MusicLibrary) {
    music_filters(ui, app_state, library);

    let mut songs = library.songs.values().collect::<Vec<_>>();

    songs.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(*a, *b));
    

    for song in &songs {
        let MusicFilters { tags, artists } = &app_state.music_filters;

        if  !tags.iter().all(|tag| song.tags.contains(tag))
            || !artists.is_empty() && !artists.contains(&song.credit_id) {
                continue
        }

        layout::add_music_button(ui, app_state, song);
    }
}

fn music_filters(ui: &mut Ui, app_state: &mut AppState, library: &MusicLibrary) {
    let songs_matching_tags = library.songs.values()
        .filter(|song| app_state.music_filters.tags.is_empty() || app_state.music_filters.tags.iter().all(|tag| song.tags.contains(tag)))
        .collect::<Vec<_>>();

    let available_artists = songs_matching_tags.iter()
        .map(|song| song.credit_id)
        .unique()
        .flat_map(|id| library.credits.get(&id))
        .sorted_unstable_by_key(|credit| &credit.name);

    let available_tags = songs_matching_tags.iter()
        .filter(|song| app_state.music_filters.artists.is_empty() || app_state.music_filters.artists.contains(&song.credit_id))
        .flat_map(|song| &song.tags)
        .unique()
        .flat_map(|id| library.tags.get(id))
        .sorted_unstable_by_key(|tag| &tag.name);

    // TODO specky would you like to add song count and tag count and artist count etc for example Tags (1) → Action [5]
    ui.horizontal(|ui| {
        ComboBox::from_id_source("music_tags_dropdown")
            .selected_text("Tags")
            .show_ui(ui, |ui| {
                for tag in available_tags {
                    let mut has_tag = app_state.music_filters.tags.contains(&tag.id);
                    ui.checkbox(&mut has_tag, &tag.name);
                    if has_tag {
                        app_state.music_filters.tags.insert(tag.id);
                    } else {
                        app_state.music_filters.tags.remove(&tag.id);
                    }
                }
            });

        ComboBox::from_id_source("music_artists_dropdown")
            .selected_text("Artists")
            .show_ui(ui, |ui| {
                for artist in available_artists {
                    let mut has_artist = app_state.music_filters.artists.contains(&artist.id);
                    ui.checkbox(&mut has_artist, &artist.name);
                    if has_artist {
                        app_state.music_filters.artists.insert(artist.id);
                    } else {
                        app_state.music_filters.artists.remove(&artist.id);
                    }
                }
            });

        if ui.button("Reset filters").clicked() {
            app_state.music_filters.tags.clear();
            app_state.music_filters.artists.clear();
        }
    });

    ui.separator();
}
