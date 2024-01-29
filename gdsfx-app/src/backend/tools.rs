use std::{thread, fs, time::Instant};

use eframe::egui::{Ui, ProgressBar};
use gdsfx_library::{FileEntry, FileEntryKind, SfxFileEntry};
use rayon::prelude::*;

use super::AppState;

pub struct ToolProgress {
    translation_key: &'static str,
    start_time: Instant,
    finished: usize,
    total: usize,
}

impl ToolProgress {
    fn new(translation_key: &'static str, total: usize) -> Self {
        Self {
            translation_key,
            start_time: Instant::now(),
            finished: 0,
            total,
        }
    }

    pub fn show_progress(&self, ui: &mut Ui) {
        ui.label(format!("{} – {}", t!(self.translation_key), self.format_time()));

        let progress = self.finished as f32 / self.total as f32;
        let text = format!("{}/{} ({:.2}%)", self.finished, self.total, progress * 100.0);
        ui.add(ProgressBar::new(progress).text(text));
    }

    fn format_time(&self) -> String {
        let elapsed_seconds = self.start_time.elapsed().as_secs();

        let seconds = elapsed_seconds % 60;
        let minutes = (elapsed_seconds / 60) % 60;
        let hours = minutes / 60;

        if hours > 0 {
            format!("{hours:02}:{minutes:02}:{seconds:02}")
        } else {
            format!("{minutes:02}:{seconds:02}")
        }
    }
}

impl AppState {
    pub fn download_multiple_sfx(&self, translation_key: &'static str, files: Vec<impl FileEntry + 'static>) {
        if !self.is_gd_folder_valid() { return }

        let progress = self.tool_progress.clone();
        *progress.lock() = Some(ToolProgress::new(translation_key, files.len()));

        let cache = match files[0].kind() {
            FileEntryKind::Sound => self.sfx_cache.clone(),
            FileEntryKind::Song => self.music_cache.clone(),
        };
        let gd_folder = self.settings.gd_folder.clone();
        let downloaded_sfx = self.downloaded_sfx.clone();
    
        thread::spawn(move || {
            files.into_par_iter().try_for_each(|file| {
                let id = file.id();
                let file_entry = SfxFileEntry::new(id);
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

        let gd_folder = self.settings.gd_folder.clone();
        let downloaded_sfx = self.downloaded_sfx.clone();

        thread::spawn(move || {
            let ids = downloaded_sfx.lock().clone();
            ids.into_iter().try_for_each(|id| {
                if SfxFileEntry::new(id).try_delete_file(&gd_folder).is_ok() {
                    downloaded_sfx.lock().remove(&id);
                }
                progress.lock().as_mut().map(|progress| progress.finished += 1)
            });

            *progress.lock() = None;
        });
    }
}
