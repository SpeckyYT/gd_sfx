// DISCALIMER: this file contains the worst code and mispells you've ever seen
// make sure to contribute to fix that

use std::time::Instant;

use reqwest::blocking::Client;
use reqwest::header::{USER_AGENT, CONTENT_TYPE};

use crate::encoding::*;
use crate::gui::{GdSfx, VersionType};
use crate::library::LibraryEntry;

const GET_CUSTOM_CONTENT_URL: &str = "https://www.boomlings.com/database/getCustomContentURL.php";
const ENDPOINT_SFX_VERSION: &str = "sfx/sfxlibrary_version.txt";
const ENDPOINT_SFX_LIBRARY: &str = "sfx/sfxlibrary.dat";

impl GdSfx {
    pub fn get_cdn_url(&mut self, force: bool) -> Option<&String> {
        if !force && self.cdn_url.is_some() { return self.cdn_url.as_ref() }

        let request = Client::default()
            .post(GET_CUSTOM_CONTENT_URL)
            .header(USER_AGENT, "")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .ok()?;
    
        let cdn_url = if request.status().is_success() {
            request.text().ok()
        } else {
            None
        };

        if let Some(cdn_url) = cdn_url {
            self.cdn_url = Some(cdn_url);
            self.cdn_url.as_ref()
        } else {
            None
        }
    }

    pub fn get_sfx_version(&mut self, force: bool) -> Option<VersionType> {
        if !force && self.sfx_version.is_some() { return self.sfx_version }

        let cdn_url = self.get_cdn_url(force)?;

        let output = Client::default()
            .get(format!("{cdn_url}/{ENDPOINT_SFX_VERSION}"))
            .send()
            .ok()?
            .text()
            .ok()?
            .parse()
            .ok();

        self.sfx_version = output;

        output
    }

    pub fn get_sfx_library(&mut self, force: bool) -> Option<&LibraryEntry> {
        let client = Client::default();

        let sfx_data = client
            .get(format!("{}/{ENDPOINT_SFX_LIBRARY}", self.get_cdn_url(force)?))
                .send()
                    .unwrap()
                        .text()
                            .unwrap();

        let sfx_data_decoded = base64_decode(sfx_data.as_bytes());

        let data = zlib_decoder(&sfx_data_decoded);

        let string = std::str::from_utf8(&data).unwrap();

        let root = LibraryEntry::from_string(string);

        self.sfx_library = Some(root);
        self.sfx_library.as_ref()
    }
}
