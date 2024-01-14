use std::{sync::Arc, path::Path};

use app_state::AppState;
use eframe::{*, egui::{ViewportBuilder, IconData}};
use gdsfx_data::paths;
use library_manager::LibraryManager;

mod app_state;
mod library_manager;

mod layout;
mod tabs;

mod i18n;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see gdsfx-app/build/i18n
gdsfx_build::get_output!(include!("i18n.rs"));

// png converted into bytes by build script
// → see gdsfx-app/build/icon.rs
const ICON_BYTES: &[u8] = gdsfx_build::get_output!(include_bytes!("icon.bin"));

struct GdSfx {
    app_state: AppState,
    library_manager: LibraryManager,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;
        
        top_panel::render(ctx, &mut self.app_state);
        left_window::render(ctx, &mut self.app_state, &self.library_manager);
        right_window::render(ctx, &self.app_state, &self.library_manager);
    }
}

impl GdSfx {
    fn run() -> eframe::Result<()> {
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
            follow_system_theme: false,
            default_theme: Theme::Dark,
            hardware_acceleration: HardwareAcceleration::Preferred,

            ..Default::default()
        };
        
        eframe::run_native(paths::runtime::APP_NAME, options, Box::new(Self::load))
    }

    fn load(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
        let app_state = AppState::load();

        let gd_folder = Path::new(&app_state.settings.gd_folder);
        let library_manager = LibraryManager::load(gd_folder);

        Box::new(Self { app_state, library_manager })
    }
}

fn main() -> eframe::Result<()> {
    hide_console_window();
    GdSfx::run()
}

fn hide_console_window() {
    if !cfg!(debug_assertions) {
        #[cfg(windows)]
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}
