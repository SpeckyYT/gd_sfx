use std::sync::Arc;

use app_state::AppState;
use eframe::{*, egui::{ViewportBuilder, IconData}};
use gdsfx_data::paths;
use library_manager::LibraryManager;

mod app_state;
mod library_manager;
mod settings;
mod favorites;

mod layout;
mod tabs;

mod sorting;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see gdsfx-app/build/i18n
gdsfx_build::get_output!(include!("i18n.rs"));

const ICON_BYTES: &[u8] = gdsfx_build::get_output!(include_bytes!("normal.bin"));

struct GdSfx {
    app_state: AppState,
    library_manager: LibraryManager,
}

impl eframe::App for GdSfx {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use layout::*;
        
        top_panel::render(ctx, &mut self.app_state);
        left_window::render(ctx, self);
        right_window::render(ctx, &self.app_state, &self.library_manager);

        // self.matching_entries.borrow_mut().clear(); // filter entries again on every frame TODO
    }
}

impl GdSfx {
    fn run() -> eframe::Result<()> {
        let options = NativeOptions {
            viewport: ViewportBuilder {
                inner_size: Some(layout::DEFAULT_WINDOW_SIZE),
                min_inner_size: Some(layout::DEFAULT_WINDOW_SIZE * layout::MIN_SCALE_FACTOR),
                resizable: Some(true),
                icon: Some(Arc::new(IconData {
                    rgba: ICON_BYTES.to_vec(),
                    width: 256,
                    height: 256,
                })),

                ..Default::default()
            },
            follow_system_theme: false,
            default_theme: Theme::Dark,
            hardware_acceleration: HardwareAcceleration::Preferred,

            ..Default::default()
        };
        
        eframe::run_native(paths::runtime::APP_NAME, options, Box::new(Self::load))
    }

    fn load(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
        Box::new(Self {
            app_state: AppState::load(),
            library_manager: LibraryManager::load(),
        })
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
