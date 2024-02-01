use std::time::Duration;
use ahash::HashMap;
use anyhow::Result;

pub mod requests;
pub mod parse;
pub mod sfx;
pub mod music;
pub mod files;

pub use files::*;

pub type EntryId = u32;
pub type BytesSize = u64;

#[derive(Debug)]
pub struct SfxLibrary {
    pub root_id: EntryId,
    pub sound_ids: Vec<EntryId>,

    pub entries: HashMap<EntryId, sfx::SfxLibraryEntry>,
    pub child_map: HashMap<EntryId, Vec<EntryId>>,

    pub total_bytes: BytesSize,
    pub total_duration: Duration,

    pub credits: Vec<sfx::Credit>,
}

#[derive(Debug)]
pub struct MusicLibrary {
    pub version: EntryId,

    pub credits: HashMap<EntryId, music::Credit>,
    pub songs: HashMap<EntryId, music::Song>,
    pub tags: HashMap<music::TagId, music::Tag>,
}
