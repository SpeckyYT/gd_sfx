use std::sync::OnceLock;

use once_cell::sync::Lazy;
use reqwest::{blocking::Client, header::*};
use url::Url;

static CLIENT: Lazy<Client> = Lazy::new(Client::default);

static GET_CUSTOM_CONTENT_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://www.boomlings.com/database/getCustomContentURL.php").unwrap()
});

static CDN_URL: Lazy<Url> = Lazy::new(|| {
    let url = CLIENT
        .post(GET_CUSTOM_CONTENT_URL.as_str())
        .header(USER_AGENT, "")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .ok()
        .filter(|response| response.status().is_success())
        .and_then(|response| response.text().ok())
        .unwrap_or(String::from("https://geometrydashfiles.b-cdn.net"));

    Url::parse(&url).unwrap()
});

static SFX_URL_PATH: Lazy<Url> = Lazy::new(|| CDN_URL.join("sfx/").unwrap());

pub(crate) fn fetch_library_version() -> Option<&'static usize> {
    const SFX_VERSION_ENDPOINT: &str = "sfxlibrary_version.txt";
    static SFX_VERSION: OnceLock<Option<usize>> = OnceLock::new();

    SFX_VERSION
        .get_or_init(|| {
            let url = SFX_URL_PATH.join(SFX_VERSION_ENDPOINT).unwrap();
            CLIENT
                .get(url.as_str())
                .send().ok()?
                .text().ok()?
                .parse().ok()
        })
        .as_ref()
}

pub(crate) fn fetch_library_data() -> Option<&'static Vec<u8>> {
    const SFX_LIBRARY_ENDPOINT: &str = "sfxlibrary.dat";
    static SFX_LIBRARY_DATA: OnceLock<Option<Vec<u8>>> = OnceLock::new();

    SFX_LIBRARY_DATA
        .get_or_init(|| {
            let url = SFX_URL_PATH.join(SFX_LIBRARY_ENDPOINT).unwrap();
            CLIENT
                .get(url.as_str())
                .send().ok()?
                .bytes()
                .map(|bytes| bytes.to_vec()).ok()
        })
        .as_ref()
}
