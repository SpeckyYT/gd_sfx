use std::{path::{PathBuf, Path}, fs};

use anyhow::Result;

use crate::{music::Song, requests, sfx::SfxLibraryEntry, EntryId};

#[derive(Copy, Clone)]
pub struct SfxFileEntry(EntryId);
#[derive(Copy, Clone)]
pub struct MusicFileEntry(EntryId);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileEntryKind {
    Sound,
    Song,
}

pub trait FileEntry: Copy + Send {
    fn new(id: EntryId) -> Self;
    fn id(&self) -> EntryId;
    fn get_file_name(&self) -> String;
    fn kind(&self) -> FileEntryKind;
    fn get_path(&self, gd_folder: impl AsRef<Path>) -> PathBuf {
        gd_folder.as_ref().join(self.get_file_name())
    }
    fn file_exists(&self, gd_folder: impl AsRef<Path>) -> bool {
        self.get_path(gd_folder).exists()
    }
    fn try_read_bytes(&self, gd_folder: impl AsRef<Path>) -> Option<Vec<u8>> {
        files::read_file(self.get_path(gd_folder)).ok()
    }
    fn try_download_bytes(&self) -> Option<Vec<u8>> {
        match self.kind() {
            FileEntryKind::Sound => requests::request_sfx_file(&self.get_file_name()),
            FileEntryKind::Song => requests::request_music_file(&self.get_file_name()),
        }.ok()
            .and_then(|response| response.bytes().ok())
            .map(|bytes| bytes.to_vec())
    }
    fn try_write_bytes(&self, gd_folder: impl AsRef<Path>, bytes: Vec<u8>) -> Result<()> {
        files::write_file(self.get_path(gd_folder), bytes)
    }
    fn try_delete_file(&self, gd_folder: impl AsRef<Path>) -> Result<()> {
        Ok(fs::remove_file(self.get_path(gd_folder))?)
    }
}

impl FileEntry for SfxFileEntry {
    fn new(id: EntryId) -> Self {
        Self(id)
    }
    fn id(&self) -> EntryId {
        self.0
    }
    fn get_file_name(&self) -> String {
        format!("s{}.ogg", self.0)
    }
    fn kind(&self) -> FileEntryKind {
        FileEntryKind::Sound
    }
}

impl FileEntry for MusicFileEntry {
    fn new(id: EntryId) -> Self {
        Self(id)
    }
    fn id(&self) -> EntryId {
        self.0
    }
    fn get_file_name(&self) -> String {
        format!("{}.ogg", self.0)
    }
    fn kind(&self) -> FileEntryKind {
        FileEntryKind::Song
    }
}

impl SfxLibraryEntry {
    pub fn into_file_entry(&self) -> SfxFileEntry {
        SfxFileEntry::new(self.id)
    }
}

impl Song {
    pub fn into_file_entry(&self) -> MusicFileEntry {
        MusicFileEntry::new(self.id)
    }
}
