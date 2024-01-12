use lazy_static::lazy_static;
use stats::Centiseconds;

use crate::credits::Credit;

pub mod favorites;
pub mod sorting;
pub mod stats;
pub mod tools;

mod credits;
mod requests;

lazy_static! {
    static ref PARSE_RESULT: (LibraryEntry, Vec<Credit>) = fetch_library();

    // fuck
    pub static ref LIBRARY: LibraryEntry = PARSE_RESULT.0;
    pub static ref SFX_CREDITS: Vec<Credit> = PARSE_RESULT.1;
}

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

fn fetch_library() -> (LibraryEntry, Vec<Credit>) {
    // request
    // parse
    // idfk
    panic!()
}
