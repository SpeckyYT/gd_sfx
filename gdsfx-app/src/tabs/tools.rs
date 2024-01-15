use std::sync::Arc;

use eframe::{egui::{Ui, Context, ProgressBar, Slider}, epaint::{mutex::Mutex, Color32}};
use egui_modal::Modal;
use once_cell::sync::Lazy;

const MAX_ID_RANGE: u32 = 100000;

type LazyAss = Lazy<Arc<Mutex<Option<(u128, u128)>>>>;

static TOOL_PROGRESS: LazyAss = Lazy::new(Default::default);

static BRUTEFORCE_RANGE: Lazy<Arc<Mutex<(u32, u32)>>> = Lazy::new(|| Arc::new(Mutex::new((0, 14500))));

pub fn render(ui: &mut Ui, ctx: &Context) {
    ui.heading(t!("tools"));

    ui.add_space(10.0);

    ui.colored_label(Color32::KHAKI, t!("tools.warning.long_time"));
    ui.colored_label(Color32::KHAKI, t!("tools.warning.program_not_usable"));

    let download_select_range_modal = download_range_select_modal(ctx);

    let is_tool_running = TOOL_PROGRESS.lock().is_some();

    ui.add_space(10.0);

    if let Some((a,b)) = *TOOL_PROGRESS.lock() {
        ui.label(format!("{} – {}", t!("placeholder"), t!("tools.progress"))); // TODO show the task which is being run
        ui.add(ProgressBar::new(a as f32 / b as f32));
    } else {
        ui.label(t!("tools.instruction"));
    }

    ui.add_space(10.0);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
            *TOOL_PROGRESS.lock() = Some((69, 420)); // TODO: INSERT HERE DOWNLOADER
        }
        if ui.button(t!("tools.download_from_range")).clicked() {
            download_select_range_modal.open();
        }
    });

    ui.add_space(10.0);

    ui.add_enabled_ui(!is_tool_running, |ui| {
        if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
            *TOOL_PROGRESS.lock() = Some((69,420)); // TODO: INSERT HERE DELETER
        }
    });
}

fn download_range_select_modal(ctx: &Context) -> Modal {
    let modal = Modal::new(ctx, "download_range_select");

    modal.show(|ui| {
        modal.title(ui, t!("tools.download_from_range"));

        modal.frame(ui, |ui| {
            let mut range = BRUTEFORCE_RANGE.lock();

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
            if ui.button(t!("tools.modal.confirm")).triple_clicked() {
                *TOOL_PROGRESS.lock() = Some((69, 420)); // TODO: INSERT HERE DOWNLOADER
                modal.close();
            }
            modal.caution_button(ui, t!("tools.modal.cancel"));
        })
    });

    modal
}
