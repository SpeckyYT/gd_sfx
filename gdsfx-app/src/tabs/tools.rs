use std::{sync::Arc, fmt::Display};

use eframe::{egui::{Ui, Context, RichText, ProgressBar, Slider}, epaint::{mutex::Mutex, Color32}};
use egui_modal::Modal;
use once_cell::sync::Lazy;

const MAX_ID_RANGE: u32 = 100000;

static DOWNLOAD_PROGRESS: Lazy<Arc<Mutex<Option<(u128, u128)>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

static BRUTEFORCE_RANGE: Lazy<Arc<Mutex<(u32, u32)>>> = Lazy::new(|| Arc::new(Mutex::new((0,14500))));

pub fn render(ui: &mut Ui, ctx: &Context) {
    ui.heading(t!("tools"));

    ui.add_space(10.0);

    ui.label(RichText::new(t!("tools.warning.long_time")).color(Color32::KHAKI));
    ui.label(RichText::new(t!("tools.warning.program_not_usable")).color(Color32::KHAKI));

    ui.label(t!("tools.instruction"));

    ui.add_space(10.0);

    let download_modal = download_modal(ctx);
    let confirm_download_modal = confirm_download_modal(ctx);

    let is_download_enabled = DOWNLOAD_PROGRESS.lock().is_none();

    if let Some((a,b)) = *DOWNLOAD_PROGRESS.lock() {
        ui.add(ProgressBar::new(a as f32 / b as f32));
    } 

    ui.add_enabled_ui(is_download_enabled, |ui| {
        if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
            *DOWNLOAD_PROGRESS.lock() = Some((69, 420)); // TODO: INSERT HERE DOWNLOADER
            download_modal.open();
        }
        if ui.button(t!("tools.download_from_range")).triple_clicked() {
            confirm_download_modal.open();
        }
    });

    ui.add_space(10.0);

    if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
        println!("deletin");
    }
}

fn download_modal(ctx: &Context) -> Modal {
    create_modal(
        t!("tools.download.title"),
        "download",
        ctx,
        |ui, modal| {
            ui.heading(t!("tools.progress"));

            if let Some((a,b)) = *DOWNLOAD_PROGRESS.lock() {
                ui.add(ProgressBar::new(a as f32 / b as f32));
            } else {
                modal.close();
            }
        },
        |ui, modal| {
            if modal.caution_button(ui, t!("tools.download.close")).clicked() {
                modal.close();
            }
        },
    )
}

fn confirm_download_modal(ctx: &Context) -> Modal {
    create_modal(
        t!("tools.download.title"),
        "confirm_download",
        ctx,
        |ui, _modal| {
            let mut range = BRUTEFORCE_RANGE.lock();

            ui.label("From ID");
            ui.add(Slider::new(&mut range.0, 0..=MAX_ID_RANGE));
            range.1 = range.1.max(range.0);

            ui.add_space(10.0);

            ui.label("To ID");
            ui.add(Slider::new(&mut range.1, 0..=MAX_ID_RANGE));
            range.0 = range.0.min(range.1);
        },
        |ui, modal| {
            if modal.button(ui, t!("tools.download.confirm")).clicked() {
                *DOWNLOAD_PROGRESS.lock() = Some((69, 420)); // TODO: INSERT HERE DOWNLOADER
                download_modal(ctx).open();
            }
            modal.caution_button(ui, t!("tools.download.close"));
        },
    )
}

fn create_modal(
    title: impl Into<RichText>,
    id: impl Display,
    ctx: &Context,
    body: impl FnOnce(&mut Ui, &Modal),
    buttons: impl FnOnce(&mut Ui, &Modal),
) -> Modal {
    let modal = Modal::new(ctx, id);

    modal.show(|ui| {
        modal.title(ui, title);
        modal.frame(ui, |ui| body(ui, &modal));
        modal.buttons(ui, |ui| buttons(ui, &modal))
    });

    modal
}