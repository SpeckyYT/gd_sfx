// <version>|{credit};{credit}|{song};{song}|{tag};{tag}
// version = number
// credit = <id>,<name>,<url>,<youtube channel id>
// songs = <id>,<name>,<credit id>,<bytes>,<duration>,.{tag id}.{tag id}.
// tag = <id>,<name>

use std::time::Duration;
use std::path::Path;
use crate::MusicLibrary;
use crate::EntryId;
use crate::*;

#[derive(Debug)]
pub struct Credit {
    pub id: EntryId,
    pub name: String,
    pub url: String,
    pub yt_channel_id: String,
}

#[derive(Debug)]
pub struct Song {
    pub id: EntryId,
    pub name: String,
    pub credit_id: EntryId,
    pub bytes: i64,
    pub duration: Duration,
    pub tags: Vec<EntryId>,
}

#[derive(Debug)]
pub struct Tag {
    pub id: EntryId,
    pub name: String,
}

impl MusicLibrary {
    pub fn load(gd_folder: impl AsRef<Path>) -> Result<Self> {
        const MUSIC_LIBRARY_FILE: &str = "musiclibrary.dat";

        let file = gd_folder.as_ref().join(MUSIC_LIBRARY_FILE);

        let local_library = gdsfx_files::read_file(&file)
            .and_then(parse::parse_music_library_from_bytes);

        if !Self::should_try_update(local_library.as_ref().ok()) {
            return local_library
        }

        requests::request_music_file(MUSIC_LIBRARY_FILE)
            .and_then(|response| {
                let bytes = response.bytes()?.to_vec();
                let _ = gdsfx_files::write_file(&file, &bytes);
                parse::parse_music_library_from_bytes(bytes)
            })
            .or_else(|download_err| local_library.map_err(|_| download_err))
    }

    fn should_try_update(library: Option<&MusicLibrary>) -> bool {
        const MUSIC_VERSION_ENDPOINT: &str = "musiclibrary_version.txt";

        let Some(library) = library else { return true };

        requests::request_music_file(MUSIC_VERSION_ENDPOINT).ok()
            .and_then(|response| response.text().ok())
            .map(|version| version != library.version.to_string())
            .unwrap_or(false) // request failed, don't bother updating
    }
}