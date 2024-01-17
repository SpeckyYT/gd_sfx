use std::{str::FromStr, collections::HashMap};

use anyhow::anyhow;

use crate::*;

pub(crate) fn parse_library_from_bytes(bytes: Vec<u8>) -> Library {
    let bytes = gdsfx_files::encoding::decode(&bytes);
    let string = std::str::from_utf8(&bytes).unwrap();

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

impl FromStr for LibraryEntry {
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
                    duration: Centiseconds(parts[5].parse()?),
                },
                EntryKind::CATEGORY_KEY => EntryKind::Category,

                _ => anyhow::bail!("Unknown library entry type")
            }
        };

        Ok(entry)
    }
}

impl ToString for LibraryEntry {
    fn to_string(&self) -> String {
        let kind = match self.kind {
            EntryKind::Sound { .. } => EntryKind::SOUND_KEY,
            EntryKind::Category => EntryKind::CATEGORY_KEY,
        };

        let (bytes, duration) = match self.kind {
            EntryKind::Sound { bytes, duration } => (bytes, duration.0),
            _ => (0, 0),
        };

        let parts = [
            self.id.to_string(),
            self.name.to_string(),
            kind.to_string(),
            self.parent_id.to_string(),
            bytes.to_string(),
            duration.to_string(),
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

fn build_library(entries: Vec<LibraryEntry>, credits: Vec<Credit>) -> Library {
    let root_id = entries.first().expect("No library entries").id;
    let mut sound_ids = Vec::new();

    let mut entry_map = HashMap::new();
    let mut child_map = HashMap::new();

    let mut total_bytes = 0;
    let mut total_duration = 0;

    for entry in entries {
        if let EntryKind::Sound { bytes, duration } = &entry.kind {
            total_bytes += *bytes;
            total_duration += duration.0;
            sound_ids.push(entry.id);
        }
        
        child_map.entry(entry.parent_id)
            .or_insert(Vec::new())
            .push(entry.id);

        entry_map.insert(entry.id, entry);
    }

    Library {
        root_id,
        sound_ids,

        entries: entry_map,
        child_map,
        
        total_bytes,
        total_duration: Centiseconds(total_duration),

        credits,
    }
}
