use std::{path::PathBuf, fs, sync::OnceLock};

use gdsfx_data::paths;
use once_cell::sync::Lazy;
use stats::Centiseconds;

use crate::credits::Credit;

pub mod favorites;
pub mod sorting;
pub mod stats;
pub mod tools;

mod credits;
mod requests;
mod parse;

pub type EntryId = u32;

pub struct LibraryEntry {
    id: EntryId,
    name: String,
    parent_id: EntryId,
    kind: EntryKind,
}

pub enum EntryKind {
    Category { children: Vec<LibraryEntry> },
    Sound { bytes: i64, duration: Centiseconds },
}

static SFX_LIBRARY_FILE: Lazy<Option<PathBuf>> = Lazy::new(|| {
    paths::runtime::GD_FOLDER.as_ref()
        .map(|path| path.join("sfxlibrary.dat"))
});

pub static SFX_LIBRARY: OnceLock<LibraryEntry> = OnceLock::new();
pub static SFX_CREDITS: OnceLock<Vec<Credit>> = OnceLock::new();

fn load_library() {
    // TODO let data = requests::fetch_library_data().or_else(try_read_library_file);
}

fn try_read_library_file() -> Option<Vec<u8>> {
    SFX_LIBRARY_FILE.as_ref()
        .and_then(|file| fs::read(file).ok())
}
