use std::{path::{PathBuf, Path}, collections::HashMap, fs};

use anyhow::Result;
use stats::Centiseconds;

pub mod stats;
pub mod tools;

mod requests;
mod parse;

pub type EntryId = u32;

#[derive(Debug)]
pub struct Library {
    root_id: EntryId,
    entries: HashMap<EntryId, LibraryEntry>,
    child_map: HashMap<EntryId, Vec<EntryId>>,

    credits: Vec<Credit>,

    total_bytes: i64,
    total_duration: Centiseconds,
}

#[derive(Debug, Clone)]
pub struct LibraryEntry {
    pub id: EntryId,
    pub name: String,
    pub parent_id: EntryId,
    pub kind: EntryKind,
}

#[derive(Debug, Clone)]
pub enum EntryKind {
    Category,
    Sound { bytes: i64, duration: Centiseconds },
}

#[derive(Debug)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

impl Library {
    pub fn load(gd_folder: impl AsRef<Path>) -> Self {
        const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";
    
        let file = gd_folder.as_ref().join(SFX_LIBRARY_FILE);
    
        gdsfx_files::read_file(&file).ok()
            .map(parse::parse_library_from_bytes)
            .filter(|library| {
                requests::fetch_library_version()
                    .map(|version| version.to_string() == library.get_root().name)
                    .unwrap_or(false)
            })
            .unwrap_or_else(|| {
                let bytes = requests::fetch_library_data().unwrap();
                let _ = gdsfx_files::write_file(&file, &bytes);
                parse::parse_library_from_bytes(bytes)
            })
    }

    pub fn get_root(&self) -> &LibraryEntry {
        self.entries.get(&self.root_id).unwrap()
    }

    pub fn get_children(&self, entry: &LibraryEntry) -> impl Iterator<Item = &LibraryEntry> {
        self.child_map
            .get(&entry.id)
            .into_iter()
            .flatten()
            .flat_map(|id| self.entries.get(id))
    }

    pub fn get_credits(&self) -> &Vec<Credit> {
        &self.credits
    }

    pub fn get_total_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn get_total_bytes(&self) -> i64 {
        self.total_bytes
    }

    pub fn get_total_duration(&self) -> Centiseconds {
        self.total_duration
    }
}

impl LibraryEntry {
    fn get_file_name(&self) -> String {
        format!("s{}.ogg", self.id)
    }

    pub fn create_file_handler(&self, gd_folder: impl AsRef<Path>) -> LibraryEntryFileHandler {
        LibraryEntryFileHandler {
            entry: self.clone(),
            path: gd_folder.as_ref().join(self.get_file_name()),
        }
    }
}

pub struct LibraryEntryFileHandler {
    entry: LibraryEntry,
    path: PathBuf,
}

impl LibraryEntryFileHandler {
    pub fn file_exists(&self) -> bool {
        self.path.exists()
    }

    pub fn try_get_bytes(&self, cache: &mut HashMap<EntryId, Vec<u8>>) -> Option<Vec<u8>> {
        cache.get(&self.entry.id).cloned().or_else(|| {
            let bytes = gdsfx_files::read_file(&self.path).ok()
                .or_else(|| requests::fetch_sfx_data(&self.entry).ok());

            if let Some(bytes) = bytes.as_ref() {
                cache.insert(self.entry.id, bytes.clone());
            }

            bytes
        })
    }

    pub fn try_store_bytes(&self, cache: &mut HashMap<EntryId, Vec<u8>>) {
        if !self.file_exists() {
            if let Some(bytes) = self.try_get_bytes(cache) {
                let _ = gdsfx_files::write_file(&self.path, bytes);
            }
        }
    }

    pub fn try_delete_file(&self) {
        let _ = fs::remove_file(&self.path);
    }
}
