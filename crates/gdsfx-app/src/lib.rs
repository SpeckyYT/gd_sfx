use std::thread;

use eframe::egui;

use gdsfx_library::{MusicLibrary, SfxLibrary};

use crate::backend::{AppState, settings::PersistentSettings};

#[macro_use]
extern crate rust_i18n;

pub mod backend;
pub mod layout;
pub mod tabs;
pub mod images;
pub mod theme;
pub mod i18n;

// build script automatically reruns the i18n! macro every time locales are modified
gdsfx_build::get_output!(include!("i18n.rs"));

pub struct GdSfx {
    app_state: AppState,
    sfx_library: SfxLibrary,
    music_library: MusicLibrary,
}

impl GdSfx {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&ctx.egui_ctx);

        ctx.egui_ctx.all_styles_mut(|style| style.url_in_tooltip = true);

        let settings = PersistentSettings::load();
        rust_i18n::set_locale(&settings.locale);

        let gd_folder = settings.gd_folder.clone();

        thread::scope(|scope| {
            let sfx_library_handle = scope.spawn(||
                SfxLibrary::load(&gd_folder)
                    .expect("TODO: info screen with error message and retry button")
            );

            let music_library_handle = scope.spawn(||
                MusicLibrary::load(&gd_folder)
                    .expect("TODO: info screen with error message and retry button")
            );

            let sfx_library = sfx_library_handle.join().unwrap();
            let music_library = music_library_handle.join().unwrap();
            let app_state = AppState::load(settings, &sfx_library, &music_library);

            Self { app_state, sfx_library, music_library }
        })
    }
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;

        backend::update(ctx, &mut self.app_state);

        tabs_panel::render(ctx, &mut self.app_state);
        left_window::render(ctx, &mut self.app_state, &self.sfx_library, &self.music_library);
        right_window::render(ctx, &mut self.app_state);
        debug_window::render(ctx, &mut self.app_state);

        backend::request_optional_repaint(ctx, &mut self.app_state);
    }
}
