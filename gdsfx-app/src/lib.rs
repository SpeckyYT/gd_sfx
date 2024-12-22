use std::{thread, sync::Arc};

use backend::{AppState, settings::PersistentSettings};
use eframe::{*, egui::{ViewportBuilder, IconData}};
use gdsfx_files::paths;
use gdsfx_library::{MusicLibrary, SfxLibrary};

mod backend;
mod layout;
mod tabs;
mod images;
mod i18n;
mod theme;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see gdsfx-app/build/i18n/update.rs
//
// this also generates the following function:
// ```
// fn get_translators(locale: &str) -> &[&str] { ... }
// ```
// → see gdsfx-app/build/i18n/translators.rs
gdsfx_build::get_output!(include!("i18n.rs"));

// png icon converted into bytes by build script
// → see gdsfx-app/build/icon.rs
const ICON_BYTES: &[u8] = gdsfx_build::get_output!(include_bytes!("icon.bin"));

pub struct GdSfx {
    app_state: AppState,
    sfx_library: SfxLibrary,
    music_library: MusicLibrary,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;
        
        backend::update(ctx, &mut self.app_state);

        tabs_panel::render(ctx, &mut self.app_state);
        left_window::render(ctx, &mut self.app_state, &self.sfx_library, &self.music_library);
        right_window::render(ctx, &mut self.app_state, &self.sfx_library, &self.music_library);

        backend::request_optional_repaint(ctx, &mut self.app_state);
    }
}

impl GdSfx {
    pub fn run() -> Result<()> {
        let icon = IconData {
            rgba: ICON_BYTES.to_vec(),
            width: gdsfx_build::ICON_WIDTH,
            height: gdsfx_build::ICON_HEIGHT,
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
            paths::runtime::APP_NAME,
            options,
            Box::new(|cc| Ok(Self::load(cc)))
        )
    }

    fn load(ctx: &eframe::CreationContext) -> Box<dyn eframe::App> {
        egui_extras::install_image_loaders(&ctx.egui_ctx);

        let settings = PersistentSettings::load();
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

            Box::new(Self { app_state, sfx_library, music_library })
        })
    }
}
