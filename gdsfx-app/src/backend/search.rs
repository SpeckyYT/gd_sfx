use std::{cmp::Ordering, time::Duration};

use ahash::HashSet;
use gdsfx_library::{music::{Song, TagId}, sfx::{EntryKind, SfxLibraryEntry}, BytesSize, EntryId};
use strum::EnumIter;

use crate::localized_enum;

#[derive(Default, Debug)]
pub struct SearchSettings {
    pub search_query: String,
    pub sorting_mode: SortingMode,
    pub show_downloaded: bool,
}

localized_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
    pub enum SortingMode = "search.sort" {
        #[default]
        NameInc = "name.ascending",      // a - z
        NameDec = "name.descending",     // z - a
        LengthInc = "length.ascending",  // 0.00 - 1.00
        LengthDec = "length.descending", // 1.00 - 0.00
        IdInc = "id.ascending",          // 0 - 9
        IdDec = "id.descending",         // 9 - 0
        SizeInc = "size.ascending",      // 0kb - 9kb
        SizeDec = "size.descending",     // 9kb - 0kb
    }
}

impl SortingMode {
    pub fn compare_entries(&self, a: &impl EntrySorting, b: &impl EntrySorting) -> Ordering {
        b.is_category().cmp(&a.is_category()) // categories on top
            .then(match self {
                Self::NameInc => a.get_name().cmp(b.get_name()),
                Self::NameDec => b.get_name().cmp(a.get_name()),
                Self::LengthInc => a.get_duration().cmp(&b.get_duration()),
                Self::LengthDec => b.get_duration().cmp(&a.get_duration()),
                Self::IdInc => a.get_id().cmp(&b.get_id()),
                Self::IdDec => b.get_id().cmp(&a.get_id()),
                Self::SizeInc => a.get_bytes().cmp(&b.get_bytes()),
                Self::SizeDec => b.get_bytes().cmp(&a.get_bytes()),
            })
    }
}

#[derive(Default, Debug)]
pub struct MusicFilters {
    pub artists: HashSet<EntryId>,
    pub tags: HashSet<TagId>,
}

pub trait EntrySorting {
    fn get_name(&self) -> &str;
    fn get_id(&self) -> EntryId;
    fn get_duration(&self) -> Duration;
    fn get_bytes(&self) -> BytesSize;
    fn is_category(&self) -> bool { false }
}

impl EntrySorting for Song {
    fn get_name(&self) -> &str { &self.name }
    fn get_id(&self) -> EntryId { self.id }
    fn get_duration(&self) -> Duration { self.duration }
    fn get_bytes(&self) -> BytesSize { self.bytes }
}

impl EntrySorting for SfxLibraryEntry {
    fn get_name(&self) -> &str { &self.name }
    fn get_id(&self) -> EntryId { self.id }
    fn get_duration(&self) -> Duration { self.duration().unwrap_or_default() }
    fn get_bytes(&self) -> BytesSize { self.bytes().unwrap_or_default() }
    fn is_category(&self) -> bool { matches!(self.kind, EntryKind::Category) }
}
