use std::{str::FromStr, collections::HashMap};

use anyhow::anyhow;

use crate::{*, stats::Centiseconds};

pub(crate) fn parse_library_from_bytes(bytes: Bytes) -> Library {
    let bytes = gdsfx_data::encoding::decode(&bytes);
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

impl FromStr for LibraryEntry {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {        
        let parts @ [id, name, kind, parent_id, ..]: [&str; 6] = string
            .split(',')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|vec| anyhow!("Invalid library entry data: {vec:?}"))?;

        let entry = Self {
            id: EntryId(id.parse()?),
            name: name.to_string(),
            parent_id: EntryId(parent_id.parse()?),

            kind: match kind {
                "0" => EntryKind::Sound {
                    bytes: parts[4].parse()?,
                    duration: Centiseconds(parts[5].parse()?),
                },
                "1" => EntryKind::Category { children: Vec::new() },

                _ => anyhow::bail!("Unknown library entry type")
            }
        };

        Ok(entry)
    }
}

impl ToString for LibraryEntry {
    fn to_string(&self) -> String {
        let kind = match self.kind {
            // TODO logic for this and the match kind above should be combined
            EntryKind::Sound { .. } => "0",
            EntryKind::Category { .. } => "1",
        };

        let (bytes, duration) = match &self.kind {
            EntryKind::Sound { bytes, duration } => (*bytes, duration.0),
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
    let mut map = entries.iter()
        .map(|entry| (entry.id, entry.clone()))
        .collect::<HashMap<_, _>>();

    for entry in &entries {
        map.entry(entry.parent_id).and_modify(|parent| {
            if let EntryKind::Category { children } = &mut parent.kind {
                children.push(entry.id);
            }
        });
    }

    Library {
        root_id: entries.into_iter().next().expect("No library entries").id,
        entries: map,
        credits
    }
}
