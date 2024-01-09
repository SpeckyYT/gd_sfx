use std::{sync::Arc, thread::{JoinHandle, self}, fs};

use eframe::epaint::mutex::Mutex;
use lazy_static::lazy_static;
use rayon::prelude::*;

use crate::{library::LibraryEntry, stats, util::GD_FOLDER};

pub mod gui;

const BRUTEFORCE_MIN: usize = 0;
const BRUTEFORCE_MAX: usize = 14500;
const BRUTEFORCE_COUNT: usize = BRUTEFORCE_MAX - BRUTEFORCE_MIN;

#[derive(Default)]
pub struct DownloadProgress {
    pub handle: Option<JoinHandle<()>>,

    pub done: usize,
    pub remaining: usize,
}

lazy_static! {
    pub static ref DOWNLOAD_PROGRESS: Arc<Mutex<DownloadProgress>> = Default::default();
}

pub fn download_everything(library: LibraryEntry) {
    fn get_sounds(library: LibraryEntry) -> Vec<LibraryEntry> {
        match library {
            LibraryEntry::Category { children, .. } =>
                children.into_iter().flat_map(get_sounds).collect(),
            LibraryEntry::Sound { .. } =>
                vec![library],
        }
    }

    start_process(|| {
        let all_sfx = get_sounds(library);

        DOWNLOAD_PROGRESS.lock().remaining = all_sfx.len();

        all_sfx.into_par_iter()
            .for_each(|entry| {
                entry.download_and_store();
                DOWNLOAD_PROGRESS.lock().done += 1;
            });
    });
}

pub fn bruteforce_everything() {
    start_process(|| {
        let range = BRUTEFORCE_MIN..BRUTEFORCE_MAX;

        DOWNLOAD_PROGRESS.lock().remaining = BRUTEFORCE_COUNT;

        range.into_par_iter()
            .for_each(|id| {
                let filename = format!("s{id}.ogg");
                let filepath = GD_FOLDER.join(filename);

                if !filepath.exists() {
                    let sfx = LibraryEntry::Sound {
                        id: id as u32,
                        bytes: 0,
                        duration: 0,
                        enabled: false,
                        name: "".to_string(),
                        parent: 0,
                    };
                    sfx.download_and_store();
                }

                DOWNLOAD_PROGRESS.lock().done += 1;
            })
    });
}

pub fn delete_everything() {
    start_process(|| {
        stats::add_existing_sfx_files();
        let existing_sound_files = stats::EXISTING_SOUND_FILES.lock();
        
        DOWNLOAD_PROGRESS.lock().remaining = existing_sound_files.len();
        
        for id in existing_sound_files.iter() {
            let filename = format!("s{id}.ogg");
            let filepath = GD_FOLDER.join(filename);
    
            if filepath.exists() {
                let _ = fs::remove_file(filepath);
            }
    
            DOWNLOAD_PROGRESS.lock().done += 1;
        }
    });
}

fn start_process(process: impl FnOnce() + Send + 'static) {
    *DOWNLOAD_PROGRESS.lock() = DownloadProgress {
        handle: Some(thread::spawn(process)),
        ..Default::default()
    };
}
