use std::time::Duration;

use eframe::egui::{CollapsingHeader, ComboBox, Ui};
use gdsfx_library::music::Song;
use gdsfx_library::{EntryId, MusicLibrary, SfxLibrary};
use gdsfx_library::sfx::{SfxLibraryEntry, EntryKind};
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::backend::search::{ListedMode, MusicFilters};
use crate::backend::LibraryPage;
use crate::i18n::LocalizedEnum;
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

    ui.separator();

    ui.add_enabled_ui(enabled, |ui| {
        CollapsingHeader::new(t!("library.unlisted_sfx"))
            .open((!enabled || collapse_all).then_some(false))
            .show(ui, |ui| {
                unlisted_sounds.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));
                for entry in unlisted_sounds {
                    layout::add_sfx_button(ui, app_state, library, &entry);
                }
            })
            .header_response
            .on_disabled_hover_text(t!("library.unlisted_sfx.hint", tool = t!("tools.download_from_range")))
    });
}

fn render_sfx_recursive(ui: &mut Ui, app_state: &mut AppState, library: &SfxLibrary, mut entries: Vec<&SfxLibraryEntry>, collapse_all: bool) {
    entries.sort_by(|&a, &b| app_state.search_settings.sorting_mode.compare_entries(a, b));
    for entry in entries {
        match entry.kind {
            EntryKind::Category => {
                let enabled = app_state.is_matching_entry(entry, library);
                if !enabled && app_state.settings.search_filter_mode == SearchFilterMode::Hide { continue }

                ui.add_enabled_ui(enabled, |ui| {
                    CollapsingHeader::new(&entry.name)
                        .open((!enabled || collapse_all).then_some(false))
                        .show(ui, |ui| {
                            render_sfx_recursive(ui, app_state, library, library.iter_children(entry).collect(), collapse_all);
                        })
                });
            }
            EntryKind::Sound { .. } => layout::add_sfx_button(ui, app_state, library, entry),
        }
    }
}

fn render_music_library(ui: &mut Ui, app_state: &mut AppState, library: &MusicLibrary) {
    music_filters(ui, app_state, library);

    match app_state.music_filters.listed_mode {
        ListedMode::Listed => {
            let mut songs: Vec<_> = library.songs.values().collect();
            songs.sort_by(|&a, &b| app_state.search_settings.sorting_mode.compare_entries(a, b));

            for song in &songs {
                let MusicFilters { tags, artists, .. } = &app_state.music_filters;
                if tags.iter().all(|tag| song.tags.contains(tag)) && (artists.is_empty() || artists.contains(&song.credit_id)) {
                    layout::add_music_button(ui, app_state, song);
                }
            }
        },
        ListedMode::Unlisted => {
            let mut songs: Vec<_> = app_state.unlisted_music.iter()
                .map(|&id| Song {
                    id,
                    name: id.to_string(),
                    bytes: 0,
                    credit_id: UNLISTED_ID,
                    duration: Duration::ZERO,
                    tags: Vec::new(),
                    unk1: String::new(),
                    unk2: String::new(),
                    url: String::new(),
                    unk3: String::new(),
                    unk4: String::new(),
                    unk5: String::new(),
                })
                .collect();
            songs.sort_by(|a, b| app_state.search_settings.sorting_mode.compare_entries(a, b));

            for song in songs {
                layout::add_music_button(ui, app_state, &song);
            }
        }
    }
}

fn music_filters(ui: &mut Ui, app_state: &mut AppState, library: &MusicLibrary) {
    ui.horizontal(|ui| {
        for listed_mode in ListedMode::iter() {
            ui.add_enabled_ui(
                listed_mode == ListedMode::Listed || !app_state.unlisted_music.is_empty(), // this is bad, but works ig
                |ui| {
                    ui.radio_value(
                        &mut app_state.music_filters.listed_mode,
                        listed_mode,
                        listed_mode.localize_variant()
                    )
                    .on_disabled_hover_text( // this is also bad, but also works ig
                        t!("library.unlisted_music.hint", tool = t!("tools.download_from_range"))
                    );
                }
            );
        }
    });

    if let ListedMode::Listed = app_state.music_filters.listed_mode {
        music_listed_filters(ui, app_state, library);
    }

    ui.separator();
}

fn music_listed_filters(ui: &mut Ui, app_state: &mut AppState, library: &MusicLibrary) {
    let available_songs = library.songs.values()
        .filter(|song| app_state.music_filters.tags.is_empty() || app_state.music_filters.tags.iter().all(|tag| song.tags.contains(tag)))
        .filter(|song| !app_state.search_settings.show_downloaded || app_state.is_music_downloaded(song.id))
        .collect::<Vec<_>>();

    let available_artists = available_songs.iter()
        .map(|song| song.credit_id)
        .unique()
        .flat_map(|id| library.credits.get(&id))
        .sorted_unstable_by_key(|credit| &credit.name);

    let available_tags = available_songs.iter()
        .filter(|song| app_state.music_filters.artists.is_empty() || app_state.music_filters.artists.contains(&song.credit_id))
        .flat_map(|song| &song.tags)
        .unique()
        .flat_map(|id| library.tags.get(id))
        .sorted_unstable_by_key(|tag| &tag.name);

    if !app_state.music_filters.artists.is_empty() {
        let tags_set = available_tags.clone().map(|tag| tag.id).collect();
        app_state.music_filters.tags = app_state.music_filters.tags.intersection(&tags_set).copied().collect();
    }

    // TODO specky would you like to add song count and tag count and artist count etc for example Tags (1) → Action [5]
    ui.horizontal(|ui| {
        ui.set_enabled(!available_songs.is_empty());

        ComboBox::from_id_salt("music_tags_dropdown")
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

        ComboBox::from_id_salt("music_artists_dropdown")
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
}
