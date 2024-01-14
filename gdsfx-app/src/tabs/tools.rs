use std::sync::Arc;

use eframe::{egui::{Ui, Context, RichText}, epaint::{mutex::Mutex, Color32}};
use once_cell::sync::Lazy;

pub fn render(ui: &mut Ui, ctx: &Context) {
    static BRUTEFORCE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

    ui.heading(t!("tools"));

    ui.add_space(10.0);

    ui.label(RichText::new(t!("tools.warning.long_time")).color(Color32::RED));
    ui.label(RichText::new(t!("tools.warning.program_not_usable")).color(Color32::RED));

    ui.add_space(10.0);

    if ui.button(t!("tools.download_all_sfx")).triple_clicked() {
        println!("downloadin");
    }
    ui.checkbox(&mut BRUTEFORCE.lock(), t!("tools.bruteforce_all_sfx"));

    ui.add_space(10.0);

    if ui.button(t!("tools.delete_all_sfx")).triple_clicked() {
        println!("deletin");
    }
}
