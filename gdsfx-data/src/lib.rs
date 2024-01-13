use std::{path::Path, fs::{DirEntry, File, self}, io::BufReader};

use anyhow::{Result, Context};
use serde::de::DeserializeOwned;

pub mod paths;
pub mod encoding;

pub fn read_dir(path: impl AsRef<Path>) -> Result<impl Iterator<Item = DirEntry>> {
    let path = path.as_ref();
    
    path.read_dir()
        .map(|dir| dir.flatten())
        .with_context(|| format!("Couldn't read directory {}", path.display()))
}

pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();

    let file = File::open(path)
        .with_context(|| format!("Couldn't open file {}", path.display()))?;

    let reader = BufReader::new(file);
    
    serde_json::from_reader::<_, T>(reader)
        .with_context(|| format!("Incorrect JSON in file {}", path.display()))
}

pub fn write_file(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();

    fs::write(path, contents)
        .with_context(|| format!("Couldn't write to file {}", path.display()))
}

pub fn create_parent_dirs(destination: impl AsRef<Path>) -> Result<()> {
    let destination = destination.as_ref();

    if let Some(path) = destination.parent() {
        return fs::create_dir_all(path)
            .with_context(|| format!("Couldn't create directories to path {}", destination.display()))
    }

    Ok(())
}
