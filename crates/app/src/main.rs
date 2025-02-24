use std::{thread, sync::Arc};

use eframe::{egui, HardwareAcceleration, NativeOptions};
use egui::{IconData, ViewportBuilder};

use library::{MusicLibrary, SfxLibrary};

use crate::backend::{AppState, settings::PersistentSettings};

#[macro_use]
extern crate rust_i18n;

mod backend;
mod layout;
mod tabs;
mod images;
mod theme;
mod i18n;

// build script automatically reruns the i18n! macro every time locales are modified
files::get_build_output!(include!("i18n.rs"));

// build script converts png icon into bytes
const ICON_BYTES: &[u8] = files::get_build_output!(include_bytes!("icon.bin"));

pub struct GdSfx {
    app_state: AppState,
    sfx_library: SfxLibrary,
    music_library: MusicLibrary,
}

impl GdSfx {
    fn new(ctx: &eframe::CreationContext) -> Self {
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

fn main() -> eframe::Result<()> {
    hide_console_window();

    let icon = IconData {
        rgba: ICON_BYTES.to_vec(),
        width: files::consts::ICON_SIZE,
        height: files::consts::ICON_SIZE,
    };

    let viewport = ViewportBuilder {
        inner_size: Some(layout::DEFAULT_WINDOW_SIZE),
        min_inner_size: Some(layout::DEFAULT_WINDOW_SIZE * layout::MIN_SCALE_FACTOR),
        resizable: Some(true),
        icon: Some(Arc::new(icon)),

        ..Default::default()
    };

    let options = NativeOptions {
        viewport,
        hardware_acceleration: HardwareAcceleration::Preferred,
        vsync: false,

        ..Default::default()
    };

    eframe::run_native(
        files::consts::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(GdSfx::new(cc))))
    )
}

fn hide_console_window() {
    #[cfg(not(debug_assertions))] {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}
