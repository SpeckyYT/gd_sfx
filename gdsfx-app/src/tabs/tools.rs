use eframe::{egui::{Ui, Context, Slider, Layout}, epaint::Color32, emath::Align};
use egui_modal::Modal;
use gdsfx_library::Library;

use crate::backend::AppState;

const MAX_ID_RANGE: u32 = 100000;

pub fn render(ui: &mut Ui, ctx: &Context, app_state: &mut AppState, library: &Library) {
    ui.heading(t!("tools"));

    ui.add_space(10.0);

    ui.colored_label(Color32::KHAKI, t!("tools.warning.long_time"));
    ui.colored_label(Color32::KHAKI, t!("tools.warning.program_not_usable"));

    let download_select_range_modal = download_range_select_modal(ctx, app_state);

    ui.add_space(10.0);

    let is_tool_running = {
        let mut tool_progress = app_state.tool_progress.lock();

        if let Some(ref mut progress) = *tool_progress {
            progress.show_progress(ui);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                if ui.button(t!("tools.cancel")).triple_clicked() {
                    *tool_progress = None;
                }
            });
        } else {
            ui.label(t!("tools.instruction"));
        }
        
        tool_progress.is_some()
    };

    ui.add_space(10.0);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
            app_state.download_multiple_sfx("tools.download_all_sfx", library.iter_sounds().map(|entry| entry.id).collect());
        }
        if ui.button(t!("tools.download_from_range")).clicked() {
            download_select_range_modal.open();
        }
    });

    ui.add_space(10.0);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
            app_state.delete_all_sfx("tools.delete_all_sfx");
        }
    });
}

fn download_range_select_modal(ctx: &Context, app_state: &mut AppState) -> Modal {
    let modal = Modal::new(ctx, "download_range_select");

    modal.show(|ui| {
        modal.title(ui, t!("tools.download_from_range"));

        modal.frame(ui, |ui| {
            let range = &mut app_state.download_id_range;

            let from_slider = Slider::new(&mut range.0, 0..=MAX_ID_RANGE)
                .text(t!("tools.download_from_range.from_id"));

            ui.add(from_slider);
            range.1 = range.1.max(range.0);

            ui.add_space(10.0);

            let to_slider = Slider::new(&mut range.1, 0..=MAX_ID_RANGE)
                .text(t!("tools.download_from_range.to_id"));

            ui.add(to_slider);
            range.0 = range.0.min(range.1);
        });

        modal.buttons(ui, |ui| {
            if ui.button(t!("tools.confirm")).triple_clicked() {
                let range = app_state.download_id_range;
                app_state.download_multiple_sfx("tools.download_from_range", (range.0..=range.1).collect());
                modal.close();
            }
            modal.caution_button(ui, t!("tools.cancel"));
        })
    });

    modal
}
