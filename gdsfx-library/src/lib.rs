use std::{path::PathBuf, fs, collections::HashMap, ops::Deref, fmt};

use gdsfx_data::paths;
use once_cell::sync::Lazy;
use stats::Centiseconds;

pub mod favorites;
pub mod sorting;
pub mod stats;
pub mod tools;

mod requests;
mod parse;

type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct Library {
    root_id: EntryId,
    entries: HashMap<EntryId, LibraryEntry>,

    credits: Vec<Credit>,
}

impl Library {
    pub fn get_root(&self) -> &LibraryEntry {
        self.get_entry(self.root_id)
    }

    pub fn get_entry(&self, id: EntryId) -> &LibraryEntry {
        self.entries.get(&id).expect("Entries shouldn't contain any non-existent IDs")
    }

    pub fn get_credits(&self) -> &Vec<Credit> {
        &self.credits
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
    Category { children: Vec<EntryId> },
    Sound { bytes: i64, duration: Centiseconds },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EntryId(u32);

impl fmt::Display for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

fn try_read_file() -> Option<Bytes> {
    const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";
    
    static SFX_LIBRARY_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
        paths::runtime::GD_FOLDER.as_ref()
            .map(|path| path.join(SFX_LIBRARY_FILE))
    });

    SFX_LIBRARY_PATH.as_ref()
        .and_then(|path| fs::read(path).ok())
}

pub fn load_library() -> Library {
    let bytes = try_read_file()
    // TODO check sfx version
        .or_else(requests::fetch_library_data)
        .unwrap();

    let bytes = gdsfx_data::encoding::decode(&bytes);
    let string = std::str::from_utf8(&bytes).unwrap();

    parse::parse_library_string(string)
}
