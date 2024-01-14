use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{layout, app_state::AppState, library_manager::LibraryManager};

pub fn render(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager) {
    layout::add_search_area(ui, app_state);
    render_recursive(ui, app_state, library_manager, library_manager.library.get_root().clone());
}

fn render_recursive(ui: &mut Ui, app_state: &mut AppState, library_manager: &LibraryManager, entry: LibraryEntry) {
    match &entry.kind {
        EntryKind::Category => {
            for child in library_manager.library.get_children(&entry) {
                render_recursive(ui, app_state, library_manager, child.clone());
            }
        },
        EntryKind::Sound { .. } => {
            if app_state.settings.is_favorite(entry.id) {
                layout::add_sfx_button(ui, app_state, library_manager, entry);
            }
        },
    }
}
