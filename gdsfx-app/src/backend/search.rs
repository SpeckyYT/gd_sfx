use std::{cmp::Ordering, time::Duration};

use gdsfx_library::{LibraryEntry, EntryKind};
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
    pub fn comparator<>(&self) -> impl FnMut(&LibraryEntry, &LibraryEntry) -> Ordering + '_ {
        fn is_category(entry: &LibraryEntry) -> bool {
            matches!(entry.kind, EntryKind::Category)
        }
    
        fn get_duration(entry: &LibraryEntry) -> Duration {
            match entry.kind {
                EntryKind::Sound { duration, .. } => duration,
                _ => Duration::ZERO,
            }
        }
    
        fn get_bytes(entry: &LibraryEntry) -> i64 {
            match entry.kind {
                EntryKind::Sound { bytes, .. } => bytes,
                _ => 0,
            }
        }
        
        move |a, b| is_category(b).cmp(&is_category(a)) // categories on top
            .then(match self {
                Sorting::Default => Ordering::Equal,
                Sorting::NameInc => a.name.cmp(&b.name),
                Sorting::NameDec => b.name.cmp(&a.name),
                Sorting::LengthInc => get_duration(a).cmp(&get_duration(b)),
                Sorting::LengthDec => get_duration(b).cmp(&get_duration(a)),
                Sorting::IdInc => a.id.cmp(&b.id),
                Sorting::IdDec => b.id.cmp(&a.id),
                Sorting::SizeInc => get_bytes(a).cmp(&get_bytes(b)),
                Sorting::SizeDec => get_bytes(b).cmp(&get_bytes(a)),
            })
    }
}
