use std::{path::PathBuf, fs, collections::HashMap, fmt, sync::Arc, ops::Deref};

use gdsfx_data::paths;
use once_cell::sync::Lazy;
use stats::Centiseconds;
use lazy_static::lazy_static;
use eframe::epaint::mutex::Mutex;

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

impl Deref for EntryId {
    type Target = u32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
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

impl LibraryEntry {
    pub fn get_file_data(&self) -> Option<Vec<u8>> {
        lazy_static! {
            static ref CACHED_SFX: Arc<Mutex<HashMap<EntryId, Vec<u8>>>> = Default::default();
        }

        match CACHED_SFX.lock().get(&self.id) {
            Some(data) => Some(data.clone()),
            None => {
                let data = match self.get_file_path().map(fs::read) {
                    Some(Ok(data)) => data,
                    Some(Err(_)) | None => {
                        match requests::fetch_sfx_data(self) {
                            None => return None,
                            Some(data) => {
                                if let Some(path) = self.get_file_path() {
                                    let _ = fs::write(path, &data);
                                }

                                data
                            },
                        }
                    },
                };

                CACHED_SFX.lock().insert(self.id, data.clone());

                Some(data)
            },
        }
    }
    
    pub fn get_file_path(&self) -> Option<PathBuf> {
        paths::runtime::GD_FOLDER.as_ref().map(|v| v.join(self.get_file_name()))
    }

    pub fn get_file_name(&self) -> String {
        format!("s{}.ogg", self.id)
    }
}
