use std::str::FromStr;
use ahash::{HashMap, HashMapExt};
use anyhow::{anyhow, Context};
use crate::sfx::*;

use crate::*;

pub(crate) fn parse_library_from_bytes(bytes: Vec<u8>) -> Result<SfxLibrary> {
    let bytes = gdsfx_files::encoding::decode(&bytes);
    let string = std::str::from_utf8(&bytes)?;

    let (library_string, credits_string) = string
        .split_once('|')
        .unwrap_or((string, ""));

    fn parse_semicolon_separated<T: FromStr>(string: &str) -> Vec<T> {
        string.split(';')
            .flat_map(T::from_str)
            .collect()
    }

    let entries = parse_semicolon_separated(library_string);
    let credits = parse_semicolon_separated(credits_string);

    build_library(entries, credits)
}

impl EntryKind {
    const SOUND_KEY: &'static str = "0";
    const CATEGORY_KEY: &'static str = "1";
}

impl FromStr for SfxLibraryEntry {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {        
        let parts @ [id, name, kind, parent_id, ..]: [&str; 6] = string
            .split(',')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|vec| anyhow!("Invalid library entry data: {vec:?}"))?;

        let entry = Self {
            id: id.parse()?,
            name: name.to_string(),
            parent_id: parent_id.parse()?,

            kind: match kind {
                EntryKind::SOUND_KEY => EntryKind::Sound {
                    bytes: parts[4].parse()?,
                    duration: Duration::from_millis(10 * parts[5].parse::<u64>()?),
                },
                EntryKind::CATEGORY_KEY => EntryKind::Category,

                _ => anyhow::bail!("Unknown library entry type")
            }
        };

        Ok(entry)
    }
}

impl ToString for SfxLibraryEntry {
    fn to_string(&self) -> String {
        let kind = match self.kind {
            EntryKind::Sound { .. } => EntryKind::SOUND_KEY,
            EntryKind::Category => EntryKind::CATEGORY_KEY,
        };

        let (bytes, duration) = match self.kind {
            EntryKind::Sound { bytes, duration } => (bytes, duration),
            _ => (0, Duration::ZERO),
        };

        let parts = [
            self.id.to_string(),
            self.name.to_string(),
            kind.to_string(),
            self.parent_id.to_string(),
            bytes.to_string(),
            (duration.as_millis() / 10).to_string(),
        ];
        
        parts.join(",")
    }
}

impl FromStr for Credit {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .split_once(',')
            .map(|(name, link)| Self {
                name: name.to_string(),
                link: link.to_string(),
            })
            .ok_or(anyhow!("Credits must have format \"name,link\", found {string}"))
    }
}

fn build_library(entries: Vec<SfxLibraryEntry>, credits: Vec<Credit>) -> Result<SfxLibrary> {
    // TODO: can the root id be (reasonably) evaluated programatically?
    let root_id = entries.first().context("No library entries")?.id;
    let mut sound_ids = Vec::new();

    let mut entry_map = HashMap::new();
    let mut child_map = HashMap::new();

    let mut total_bytes = 0;
    let mut total_duration = Duration::ZERO;

    for entry in entries {
        if let EntryKind::Sound { bytes, duration } = &entry.kind {
            total_bytes += *bytes;
            total_duration += *duration;

            sound_ids.push(entry.id);
        }
        
        child_map.entry(entry.parent_id)
            .or_insert(Vec::new())
            .push(entry.id);

        entry_map.insert(entry.id, entry);
    }

    Ok(SfxLibrary {
        root_id,
        sound_ids,

        entries: entry_map,
        child_map,
        
        total_bytes,
        total_duration,

        credits,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_library_entry() {
        const FIRE_IN_THE_HOLE: &str = "4451,Fire In The Hole,0,4442,29496,187";

        let entry = SfxLibraryEntry::from_str(FIRE_IN_THE_HOLE).unwrap();
        assert_eq!(entry, SfxLibraryEntry {
            id: 4451,
            name: "Fire In The Hole".to_string(),
            parent_id: 4442,
            kind: EntryKind::Sound {
                bytes: 29496,
                duration: Duration::from_millis(187 * 10),
            }
        });

        let string = entry.to_string();
        assert_eq!(string, FIRE_IN_THE_HOLE);
    }

    #[test]
    fn test_parse_credit() {
        const SHARKS_CREDIT: &str = "Sharks,https://www.sharkstunes.com";

        let credit = Credit::from_str(SHARKS_CREDIT).unwrap();
        assert_eq!(credit, Credit {
            name: "Sharks".to_string(),
            link: "https://www.sharkstunes.com".to_string(),
        });
    }
}
