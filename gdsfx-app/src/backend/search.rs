use std::cmp::Ordering;

use ahash::HashSet;
use gdsfx_library::{music::TagId, EntryId, SortingGetter};
use strum::EnumIter;

use crate::localized_enum;

#[derive(Default, Debug)]
pub struct SearchSettings {
    pub search_query: String,
    pub sorting_mode: Sorting,
    pub show_downloaded: bool,
}

localized_enum! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
    pub enum Sorting = "search.sort" {
        #[default]
        Default = "default",
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

impl Sorting {
    pub fn compare_entries(&self, a: impl SortingGetter, b: impl SortingGetter) -> Ordering {
        b.get_is_category().cmp(&a.get_is_category()) // categories on top
            .then(match self {
                Sorting::Default => Ordering::Equal,
                Sorting::NameInc => a.get_name().cmp(&b.get_name()),
                Sorting::NameDec => b.get_name().cmp(&a.get_name()),
                Sorting::LengthInc => a.get_duration().cmp(&b.get_duration()),
                Sorting::LengthDec => b.get_duration().cmp(&a.get_duration()),
                Sorting::IdInc => a.get_id().cmp(&b.get_id()),
                Sorting::IdDec => b.get_id().cmp(&a.get_id()),
                Sorting::SizeInc => a.get_bytes().cmp(&b.get_bytes()),
                Sorting::SizeDec => b.get_bytes().cmp(&a.get_bytes()),
            })
    }
}

#[derive(Default, Debug)]
pub struct MusicFilters {
    pub artists: HashSet<EntryId>,
    pub tags: HashSet<TagId>,
}
