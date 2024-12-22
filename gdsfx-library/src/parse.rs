use std::fmt::Display;
use std::str::FromStr;
use ahash::{HashMap, HashMapExt};
use anyhow::{anyhow, Context};
use crate::sfx;
use crate::music;
use crate::sfx::SfxLibraryEntry;
use crate::*;

fn parse_semicolon_separated<T: FromStr>(string: &str) -> Vec<T> {
    string.split(';')
        .flat_map(T::from_str)
        .collect()
}

pub(crate) fn parse_sfx_library_from_bytes(bytes: Vec<u8>) -> Result<SfxLibrary> {
    let bytes = gdsfx_files::encoding::decode(&bytes);
    let string = std::str::from_utf8(&bytes)?;

    let (library_string, credits_string) = string
        .split_once('|')
        .unwrap_or((string, ""));

    let entries = parse_semicolon_separated(library_string);
    let credits = parse_semicolon_separated(credits_string);

    build_sfx_library(entries, credits)
}

pub(crate) fn parse_music_library_from_bytes(bytes: Vec<u8>) -> Result<MusicLibrary> {
    let bytes = gdsfx_files::encoding::decode(&bytes);
    let string = bytes.iter().map(|&byte| char::from(byte)).collect::<String>();

    let [version, credits, songs, tags]: [&str; 4] = string
        .split('|')
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|vec| anyhow!("Invalid library entry data: {vec:?}"))?;

    Ok(MusicLibrary {
        version: version.parse()?,
        credits: parse_semicolon_separated(credits).into_iter().map(|x: music::Credit| (x.id, x)).collect(),
        songs: parse_semicolon_separated(songs).into_iter().map(|x: music::Song| (x.id, x)).collect(),
        tags: parse_semicolon_separated(tags).into_iter().map(|x: music::Tag| (x.id, x)).collect(),
    })
}

impl sfx::EntryKind {
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
                sfx::EntryKind::SOUND_KEY => sfx::EntryKind::Sound {
                    bytes: parts[4].parse()?,
                    duration: Duration::from_millis(10 * parts[5].parse::<u64>()?),
                },
                sfx::EntryKind::CATEGORY_KEY => sfx::EntryKind::Category,

                _ => anyhow::bail!("Unknown library entry type")
            }
        };

        Ok(entry)
    }
}

impl Display for SfxLibraryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            sfx::EntryKind::Sound { .. } => sfx::EntryKind::SOUND_KEY,
            sfx::EntryKind::Category => sfx::EntryKind::CATEGORY_KEY,
        };

        let (bytes, duration) = match self.kind {
            sfx::EntryKind::Sound { bytes, duration } => (bytes, duration),
            _ => (0, Duration::ZERO),
        };

        let parts = [
            self.id.to_string(),
            self.name.to_string(),
            kind.to_string(),
            self.parent_id.to_string(),
            bytes.to_string(),
            (duration.as_millis() / 10).to_string(),
        ]
        .join(",");
        
        f.write_str(&parts)
    }
}

impl FromStr for sfx::Credit {
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

impl FromStr for music::Credit {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .split(',')
            .collect::<Vec<&str>>()
            .try_into()
            .map(|[id, name, url, yt_channel_id]: [&str; 4]| Self {
                id: id.trim().parse().unwrap_or(0),
                name: name.trim().to_string(),
                url: {
                    let url = url.trim();
                    if url.is_empty() {
                        None
                    } else {
                        urlencoding::decode(url)
                            .map(|url| url.to_string())
                            .ok()
                    }
                },
                yt_url: {
                    let yt_channel_id = yt_channel_id.trim();
                    (!yt_channel_id.is_empty()).then_some(format!("https://youtube.com/channel/{yt_channel_id}"))
                },
            })
            .ok()
            .ok_or(anyhow!("Credits must have format \"id,name,url,yt_channel_id\", found {string}"))
    }
}

impl FromStr for music::Song {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .split(',')
            .collect::<Vec<&str>>()
            .try_into()
            .map(|[id, name, credit_id, bytes, duration, tags, ncs, unk2, url, new, unk4, unk5 ]: [&str; 12]| {
                let song = Self {
                    id: id.parse().unwrap_or(0),
                    name: name.to_string(),
                    credit_id: credit_id.parse().unwrap_or(0),
                    bytes: bytes.parse().unwrap_or(0),
                    duration: Duration::from_secs(duration.parse().unwrap_or(0)),
                    tags: tags.split('.').filter_map(|s| s.parse().ok()).collect(),
                    ncs: ncs == "1",
                    unk2: unk2.to_string(),
                    url: url.to_string(),
                    new: new == "1",
                    unk4: unk4.to_string(),
                    unk5: unk5.to_string(),
                };

                song
            })
            .ok()
            .ok_or(anyhow!("Songs must have format \"id,name,credit_id,bytes,duration,tags,ncs,unk2,url,new,unk4,unk5\", found {string}"))
    }
}

impl FromStr for music::Tag {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .split_once(',')
            .map(|(id, name)| Self {
                id: id.parse().unwrap_or(0),
                name: name.to_string(),
            })
            .ok_or(anyhow!("Tags must have format \"id,name\", found {string}"))
    }
}

fn build_sfx_library(entries: Vec<SfxLibraryEntry>, credits: Vec<sfx::Credit>) -> Result<SfxLibrary> {
    // TODO: can the root id be (reasonably) evaluated programatically?
    let root_id = entries.first().context("No library entries")?.id;
    let mut sound_ids = Vec::new();

    let mut entry_map = HashMap::new();
    let mut child_map = HashMap::new();

    let mut total_bytes = 0;
    let mut total_duration = Duration::ZERO;

    for entry in entries {
        if let sfx::EntryKind::Sound { bytes, duration } = &entry.kind {
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
            kind: sfx::EntryKind::Sound {
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

        let credit = sfx::Credit::from_str(SHARKS_CREDIT).unwrap();
        assert_eq!(credit, sfx::Credit {
            name: "Sharks".to_string(),
            link: "https://www.sharkstunes.com".to_string(),
        });
    }
}
