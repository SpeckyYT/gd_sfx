use std::{path::{PathBuf, Path}, fs};

use crate::{EntryId, requests};

pub struct FileEntry(EntryId);

impl FileEntry {
    pub fn new(id: EntryId) -> Self {
        Self(id)
    }

    fn get_file_name(&self) -> String {
        format!("s{}.ogg", self.0)
    }

    fn get_path(&self, gd_folder: impl AsRef<Path>) -> PathBuf {
        gd_folder.as_ref().join(self.get_file_name())
    }

    pub fn file_exists(&self, gd_folder: impl AsRef<Path>) -> bool {
        self.get_path(gd_folder).exists()
    }

    pub fn try_read_bytes(&self, gd_folder: impl AsRef<Path>) -> Option<Vec<u8>> {
        gdsfx_files::read_file(self.get_path(gd_folder)).ok()
    }

    pub fn try_download_bytes(&self) -> Option<Vec<u8>> {
        requests::request_file(&self.get_file_name())
            .and_then(|response| response.bytes().ok())
            .map(|bytes| bytes.to_vec())
    }

    pub fn try_write_bytes(&self, gd_folder: impl AsRef<Path>, bytes: Vec<u8>) {
        let _ = gdsfx_files::write_file(self.get_path(gd_folder), bytes);
    }

    pub fn try_delete_file(&self, gd_folder: impl AsRef<Path>) {
        let _ = fs::remove_file(self.get_path(gd_folder));
    }
}
