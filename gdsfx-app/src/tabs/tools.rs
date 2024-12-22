use eframe::{egui::{Ui, Context, Slider, Layout}, emath::Align};
use egui_modal::Modal;
use gdsfx_library::{FileEntry, MusicFileEntry, MusicLibrary, SfxFileEntry, SfxLibrary};

use crate::{backend::{AppState, LibraryPage}, i18n::LocalizedEnum, layout};

pub fn render(ui: &mut Ui, ctx: &Context, app_state: &mut AppState, sfx_library: &SfxLibrary, music_library: &MusicLibrary) {
    layout::add_library_page_selection(ui, app_state);

    ui.heading(t!("tools"));

    ui.add_space(10.0);

    render_running_tool(ui, app_state);

    ui.add_space(10.0);

    let is_tool_running = app_state.is_tool_running();
    let download_select_range_modal = download_range_select_modal(ctx, app_state);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        let download_all_key = format!("tools.download_all.{}", app_state.library_page.localization_key());
        
        if ui.button(t!(&download_all_key)).triple_clicked() {
            match app_state.library_page {
                LibraryPage::Sfx =>
                    app_state.download_multiple_sfx(
                        download_all_key,
                        sfx_library.sound_ids().iter().map(|&i| SfxFileEntry::new(i)).collect(),
                    ),
                LibraryPage::Music =>
                    app_state.download_multiple_sfx(
                        download_all_key,
                        music_library.songs.keys().map(|&i| MusicFileEntry::new(i)).collect(),
                    ),
            }
        }
        if ui.button(t!("tools.download_from_range")).clicked() {
            download_select_range_modal.open();
        }
    });

    ui.add_space(10.0);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        let delete_all_key = format!("tools.delete_all.{}", app_state.library_page.localization_key());

        if ui.button(t!(&delete_all_key)).triple_clicked() {
            app_state.delete_all_sfx(delete_all_key);
        }
    });
}

fn render_running_tool(ui: &mut Ui, app_state: &mut AppState) {
    let mut tool_progress = app_state.tool_progress.lock();
    
    if let Some(ref mut progress) = *tool_progress {
        progress.show_progress(ui);
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            if layout::add_caution_button(ui, t!("tools.stop")).triple_clicked() {
                *tool_progress = None;
            }
        });
    } else {
        ui.label(t!("tools.instruction"));
    }
}

fn download_range_select_modal(ctx: &Context, app_state: &mut AppState) -> Modal {
    let modal = Modal::new(ctx, "download_range_select");

    modal.show(|ui| {
        modal.title(ui, t!("tools.download_from_range"));

        modal.frame(ui, |ui| {
            let (min_id_range, max_id_range, range) = match app_state.library_page {
                LibraryPage::Sfx => (0, 200000, &mut app_state.download_id_range_sfx),
                LibraryPage::Music => (10000000, 10100000, &mut app_state.download_id_range_music),
            };

            let from_slider = Slider::new(&mut range.0, min_id_range..=max_id_range)
                .text(t!("tools.download_from_range.from_id"));

            ui.add(from_slider);
            range.1 = range.1.max(range.0);

            ui.add_space(10.0);

            let to_slider = Slider::new(&mut range.1, min_id_range..=max_id_range)
                .text(t!("tools.download_from_range.to_id"));

            ui.add(to_slider);
            range.0 = range.0.min(range.1);
        });

        modal.buttons(ui, |ui| {
            if ui.button(t!("tools.confirm")).triple_clicked() {
                let download_from_range_string = "tools.download_from_range".to_string();
                match app_state.library_page {
                    LibraryPage::Sfx => {
                        let range = app_state.download_id_range_sfx;
                        app_state.download_multiple_sfx(
                            download_from_range_string,
                            (range.0..=range.1).map(SfxFileEntry::new).collect()
                        );
                    },
                    LibraryPage::Music => {
                        let range = app_state.download_id_range_music;
                        app_state.download_multiple_sfx(
                            download_from_range_string,
                            (range.0..=range.1).map(MusicFileEntry::new).collect()
                        );
                    }
                }
                modal.close();
            }
            modal.caution_button(ui, t!("tools.cancel"));
        })
    });

    modal
}
