use std::{path::Path, time::Duration};
use ahash::HashMap;
use anyhow::Result;

mod requests;
mod parse;

mod files;
pub use files::FileEntry;

pub type EntryId = u32;

#[derive(Debug)]
pub struct Library {
    root_id: EntryId,
    sound_ids: Vec<EntryId>,

    entries: HashMap<EntryId, LibraryEntry>,
    child_map: HashMap<EntryId, Vec<EntryId>>,

    total_bytes: i64,
    total_duration: Duration,

    credits: Vec<Credit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryEntry {
    pub id: EntryId,
    pub name: String,
    pub parent_id: EntryId,
    pub kind: EntryKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryKind {
    Category,
    Sound { bytes: i64, duration: Duration },
}

#[derive(Debug, PartialEq)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

impl Library {
    pub fn load(gd_folder: impl AsRef<Path>) -> Result<Self> {
        const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";

        let file = gd_folder.as_ref().join(SFX_LIBRARY_FILE);

        let local_library = gdsfx_files::read_file(&file)
            .and_then(parse::parse_library_from_bytes);

        if !Self::should_try_update(local_library.as_ref().ok()) {
            return local_library
        }

        requests::request_file(SFX_LIBRARY_FILE)
            .and_then(|response| {
                let bytes = response.bytes()?.to_vec();
                let _ = gdsfx_files::write_file(&file, &bytes);
                parse::parse_library_from_bytes(bytes)
            })
            .or_else(|download_err| local_library.map_err(|_| download_err))
    }

    fn should_try_update(library: Option<&Library>) -> bool {
        const SFX_VERSION_ENDPOINT: &str = "sfxlibrary_version.txt";

        let Some(library) = library else { return true };

        requests::request_file(SFX_VERSION_ENDPOINT).ok()
            .and_then(|response| response.text().ok())
            .map(|version| version != library.get_version())
            .unwrap_or(false) // request failed, don't bother updating
    }

    pub fn get_root(&self) -> &LibraryEntry {
        self.entries.get(&self.root_id).expect("Root ID not in library")
    }

    pub fn entries(&self) -> &HashMap<EntryId, LibraryEntry> {
        &self.entries
    }

    pub fn iter_children(&self, entry: &LibraryEntry) -> impl Iterator<Item = &LibraryEntry> {
        self.child_map
            .get(&entry.id)
            .into_iter()
            .flatten()
            .flat_map(|id| self.entries.get(id))
    }

    pub fn sound_ids(&self) -> &Vec<EntryId> {
        &self.sound_ids
    }

    pub fn iter_sounds(&self) -> impl Iterator<Item = &LibraryEntry> {
        self.sound_ids.iter().flat_map(|id| self.entries.get(id))
    }

    pub fn total_bytes(&self) -> i64 {
        self.total_bytes
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    pub fn get_version(&self) -> &str {
        &self.get_root().name
    }

    pub fn credits(&self) -> &Vec<Credit> {
        &self.credits
    }
}
