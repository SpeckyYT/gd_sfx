use std::str::FromStr;

use anyhow::anyhow;

use crate::{LibraryEntry, Credit, EntryKind, stats::Centiseconds, Library};

pub(crate) fn parse_library_string(string: &str) -> Library {
    let (library_string, credits_string) = string
        .split_once('|')
        .unwrap_or((string, ""));

    fn parse_semicolon_separated<T: FromStr>(string: &str) -> Vec<T> {
        string.split(';')
            .flat_map(T::from_str)
            .collect()
    }
    
    Library {
        library: build_library_tree(parse_semicolon_separated(library_string)),
        credits: parse_semicolon_separated(credits_string),
    }
}

impl FromStr for LibraryEntry {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts = string.split(',').collect::<Vec<_>>();
        let parts @ [id, name, kind, parent_id, ..]: [&str; 6] =
            parts.try_into().map_err(|e| anyhow!("Invalid library entry data: {e:?}"))?;

        let entry = Self {
            id: id.parse()?,
            name: name.to_string(),
            parent_id: parent_id.parse()?,

            kind: match kind {
                "0" => EntryKind::Sound {
                    bytes: parts[4].parse()?,
                    duration: Centiseconds(parts[5].parse()?),
                },
                "1" => EntryKind::Category { children: vec![] },

                _ => anyhow::bail!("Unknown library entry type")
            }
        };

        Ok(entry)
    }
}

fn build_library_tree(entries: Vec<LibraryEntry>) -> LibraryEntry {
    todo!()
}

impl FromStr for Credit {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string.split_once(',')
            .map(|(name, link)| Self {
                name: name.to_string(),
                link: link.to_string(),
            })
            .ok_or(anyhow!("Credit must have format \"name,link\", found {string}"))
    }
}
