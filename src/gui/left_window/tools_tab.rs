use eframe::{egui::{Ui, Context, ProgressBar}, epaint::Color32};
use egui_modal::Modal;

use crate::{gui::GdSfx, tools::{self, *}};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, ctx: &Context) {
    let modal_generator = |title, id| -> Modal {
        let modal = Modal::new(ctx, format!("{id}_modal"));

        modal.show(|ui| {
            let DownloadProgress { ref mut handle, done, remaining } = *DOWNLOAD_PROGRESS.lock().unwrap();

            if handle.as_ref().filter(|handle| handle.is_finished()).is_some() {
                *handle = None;
                modal.close();
            }

            modal.title(ui, title);
    
            let progress = if remaining == 0 { 0.0 } else { done as f32 / remaining as f32 };
            let progress_bar = ProgressBar::new(progress)
                .animate(true)
                .text(format!("{done} / {remaining} ({:.2}%)", progress));

            ui.heading(t!("tools.progress"));

            ui.add(progress_bar);
        });

        modal
    };

    let download_modal = modal_generator(t!("tools.download_all_sfx.title"), "download");
    let delete_modal = modal_generator(t!("tools.delete_all_sfx.title"), "delete");
    
    ui.heading(t!("tools"));
    
    ui.add_space(10.0);

    ui.colored_label(Color32::RED, t!("tools.warning.long_time"));
    ui.colored_label(Color32::RED, t!("tools.warning.program_not_usable"));

    ui.add_space(10.0);

    ui.label(t!("tools.instruction"));

    ui.add_space(10.0);

    if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
        download_modal.open();
        let library = gdsfx.sfx_library.as_ref().unwrap().sound_effects.clone();
        tools::download_everything(library);
    }
    if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
        delete_modal.open();
        tools::delete_everything();
    }
}