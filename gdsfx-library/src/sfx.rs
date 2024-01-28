use crate::parse;
use crate::requests;
use crate::BytesSize;
use crate::EntryId;
use crate::SfxLibrary;
use std::path::Path;
use std::time::Duration;
use anyhow::Result;
use ahash::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SfxLibraryEntry {
    pub id: EntryId,
    pub name: String,
    pub parent_id: EntryId,
    pub kind: EntryKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryKind {
    Category,
    Sound { bytes: BytesSize, duration: Duration },
}

#[derive(Debug, PartialEq)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

impl SfxLibrary {
    pub fn load(gd_folder: impl AsRef<Path>) -> Result<Self> {
        const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";

        let file = gd_folder.as_ref().join(SFX_LIBRARY_FILE);

        let local_library = gdsfx_files::read_file(&file)
            .and_then(parse::parse_sfx_library_from_bytes);

        if !Self::should_try_update(local_library.as_ref().ok()) {
            return local_library
        }

        requests::request_sfx_file(SFX_LIBRARY_FILE)
            .and_then(|response| {
                let bytes = response.bytes()?.to_vec();
                let _ = gdsfx_files::write_file(&file, &bytes);
                parse::parse_sfx_library_from_bytes(bytes)
            })
            .or_else(|download_err| local_library.map_err(|_| download_err))
    }

    fn should_try_update(library: Option<&SfxLibrary>) -> bool {
        const SFX_VERSION_ENDPOINT: &str = "sfxlibrary_version.txt";

        let Some(library) = library else { return true };

        requests::request_sfx_file(SFX_VERSION_ENDPOINT).ok()
            .and_then(|response| response.text().ok())
            .map(|version| version != library.get_version())
            .unwrap_or(false) // request failed, don't bother updating
    }

    pub fn get_root(&self) -> &SfxLibraryEntry {
        self.entries.get(&self.root_id).expect("Root ID not in library")
    }

    pub fn entries(&self) -> &HashMap<EntryId, SfxLibraryEntry> {
        &self.entries
    }

    pub fn iter_children(&self, entry: &SfxLibraryEntry) -> impl Iterator<Item = &SfxLibraryEntry> {
        self.child_map
            .get(&entry.id)
            .into_iter()
            .flatten()
            .flat_map(|id| self.entries.get(id))
    }

    pub fn sound_ids(&self) -> &Vec<EntryId> {
        &self.sound_ids
    }

    pub fn iter_sounds(&self) -> impl Iterator<Item = &SfxLibraryEntry> {
        self.sound_ids.iter().flat_map(|id| self.entries.get(id))
    }

    pub fn total_bytes(&self) -> BytesSize {
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

impl SfxLibraryEntry {
    pub fn bytes(&self) -> Option<BytesSize> {
        match self.kind {
            EntryKind::Category => None,
            EntryKind::Sound { bytes, .. } => Some(bytes),
        }
    }
    pub fn duration(&self) -> Option<Duration> {
        match self.kind {
            EntryKind::Category => None,
            EntryKind::Sound { duration, .. } => Some(duration),
        }
    }
}
