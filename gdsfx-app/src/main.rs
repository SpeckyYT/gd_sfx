use eframe::{egui, epaint::Vec2};
use gdsfx_data::paths;
use gdsfx_library::sorting::Sorting;
use settings::Settings;

use crate::tabs::Tab;

mod layout;
mod tabs;
mod elements;

mod settings;

// the build script reruns every time a file in the lang folder is changed
// and writes the i18n!(...) macro invocation to this file so it is always updated
// → see gdsfx-app/build/i18n
gdsfx_build::include!("i18n.rs");

#[derive(Default)]
struct GdSfx {
    tab: Tab,
    search_query: String,
    sorting: Sorting,
    settings: Settings,
}

impl GdSfx {
    fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                // TODO constants
                .with_inner_size(Vec2 { x: 800.0, y: 600.0 })
                .with_min_inner_size(Vec2 { x: 560.0, y: 420.0 }),
    
            follow_system_theme: false,
            ..Default::default()
        };
        
        eframe::run_native(paths::runtime::APP_NAME, options, Box::new(Self::load))
    }

    fn load(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
        Box::new(Self {
            settings: Settings::load_or_default(),
            ..Default::default()
        })
    }
}

impl eframe::App for GdSfx {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // top_panel::render(self, ctx);
        // left_window::render(self, ctx);
        // right_window::render(self, ctx);
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
