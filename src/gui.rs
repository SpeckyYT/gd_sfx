use eframe::{egui, NativeOptions};

use crate::library::LibraryEntry;

pub type VersionType = usize;

#[derive(Default)]
pub struct GdSfx {
    pub cdn_url: Option<String>,
    pub sfx_version: Option<VersionType>,
    pub sfx_library: Option<LibraryEntry>,
    pub library: Vec<()>,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            if ui.button("yo").clicked() {
                self.get_sfx_library(false);
            }
        });
    }
}

impl GdSfx {
    pub fn run(options: NativeOptions) {
        eframe::run_native(
            "GDSFX",
            options,
            Box::new(|_cc| Box::<Self>::default()),
        )
        .unwrap()
    }
}
