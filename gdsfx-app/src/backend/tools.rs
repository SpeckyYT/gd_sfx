use std::{thread, fs};

use eframe::egui::{Ui, ProgressBar};
use gdsfx_library::{EntryId, FileEntry};
use rayon::prelude::*;

use super::AppState;

#[derive(Default)]
pub struct ToolProgress {
    translation_key: &'static str,
    finished: usize,
    total: usize,
}

impl ToolProgress {
    fn new(translation_key: &'static str, total: usize) -> Self {
        Self { translation_key, finished: 0, total }
    }

    pub fn show_progress(&self, ui: &mut Ui) {
        ui.label(format!("{} – {}", t!(self.translation_key), t!("tools.progress")));

        let progress = self.finished as f32 / self.total as f32;
        let text = format!("{}/{} ({:.2}%)", self.finished, self.total, progress * 100.0);
        ui.add(ProgressBar::new(progress).text(text));
    }
}

impl AppState {
    pub fn download_multiple_sfx(&self, translation_key: &'static str, ids: Vec<EntryId>) {
        if !self.is_gd_folder_valid() { return }

        let progress = self.tool_progress.clone();
        *progress.lock() = Some(ToolProgress::new(translation_key, ids.len()));

        let cache = self.sfx_cache.clone();
        let gd_folder = self.settings.gd_folder.clone();
        let downloaded_sfx = self.downloaded_sfx.clone();
    
        thread::spawn(move || {
            ids.into_par_iter().try_for_each(|id| {
                let file_entry = FileEntry::new(id);
                if !file_entry.file_exists(&gd_folder) {
                    let bytes = cache.lock().get(&id).cloned()
                        .or_else(|| file_entry.try_download_bytes());

                    if let Some(bytes) = bytes {
                        if file_entry.try_write_bytes(&gd_folder, bytes).is_ok() {
                            downloaded_sfx.lock().insert(id);
                        }
                    }
                }
                progress.lock().as_mut().map(|progress| progress.finished += 1)
            });
    
            *progress.lock() = None;
        });
    }
    
    pub fn delete_all_sfx(&self, translation_key: &'static str) {
        let Ok(read_dir) = fs::read_dir(&self.settings.gd_folder) else { return };
        let read_dir = read_dir.flatten().collect::<Vec<_>>();
        
        let progress = self.tool_progress.clone();
        *progress.lock() = Some(ToolProgress::new(translation_key, read_dir.len()));

        thread::spawn(move || {
            read_dir.into_par_iter()
                .filter(|entry| {
                    entry.file_name().to_str()
                        .filter(|s| s.starts_with('s') && s.ends_with(".ogg"))
                        .map(|s| &s[1..s.len()-4])
                        .and_then(|s| s.parse::<u32>().ok())
                        .is_some()
                })
                .try_for_each(|entry| {
                    let _ = fs::remove_file(entry.path());
                    progress.lock().as_mut().map(|progress| progress.finished += 1)
                });

            *progress.lock() = None;
        });
    }
}
