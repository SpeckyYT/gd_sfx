use std::{path::{PathBuf, Path}, collections::HashMap, fs};

use anyhow::Result;
use stats::Centiseconds;

pub mod stats;
pub mod tools;

mod requests;
mod parse;

pub type EntryId = u32;

type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct Library {
    root_id: EntryId,
    entries: HashMap<EntryId, LibraryEntry>,
    child_map: HashMap<EntryId, Vec<EntryId>>,

    credits: Vec<Credit>,

    total_bytes: i64,
    total_duration: Centiseconds,
}

impl Library {
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

pub fn load_library(gd_folder: &Path) -> Library {
    const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";

    let file = gd_folder.join(SFX_LIBRARY_FILE);

    gdsfx_data::read_file(&file).ok()
        .map(parse::parse_library_from_bytes)
        .filter(|library| {
            requests::fetch_library_version()
                .map(|version| version.to_string() == library.get_root().name)
                .unwrap_or(false)
        })
        .unwrap_or_else(|| {
            let bytes = requests::fetch_library_data().unwrap();
            let _ = gdsfx_data::write_file(&file, &bytes);
            parse::parse_library_from_bytes(bytes)
        })
}

// TODO this gd_folder spam is really stupid
impl LibraryEntry {
    pub fn get_file_name(&self) -> String {
        format!("s{}.ogg", self.id)
    }

    pub fn try_get_file_path(&self, gd_folder: &Path) -> Option<PathBuf> {
        Some(gd_folder.join(self.get_file_name()))
    }

    pub fn file_exists(&self, gd_folder: &Path) -> bool {
        self.try_get_file_path(gd_folder)
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    pub fn try_get_bytes(&self, gd_folder: &Path, cache: &mut HashMap<EntryId, Bytes>) -> Option<Bytes> {
        cache.get(&self.id).cloned().or_else(|| {
            let bytes = self
                .try_get_file_path(gd_folder)
                .and_then(|path| gdsfx_data::read_file(path).ok())
                .or_else(|| requests::fetch_sfx_data(self).ok());

            if let Some(bytes) = bytes.as_ref() {
                cache.insert(self.id, bytes.clone());
            }

            bytes
        })
    }

    pub fn try_store_bytes(&self, gd_folder: &Path, cache: &mut HashMap<EntryId, Bytes>) {
        if !self.file_exists(gd_folder) {
            if let Some(path) = self.try_get_file_path(gd_folder) {
                if let Some(bytes) = self.try_get_bytes(gd_folder, cache) {
                    let _ = gdsfx_data::write_file(path, bytes);
                }
            }
        }
    }

    pub fn try_delete_file(&self, gd_folder: &Path) {
        if let Some(path) = self.try_get_file_path(gd_folder) {
            let _ = fs::remove_file(path);
        }
    }
}
