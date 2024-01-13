use once_cell::sync::Lazy;
use reqwest::{blocking::Client, header::*};
use url::Url;

use crate::Bytes;

static CLIENT: Lazy<Client> = Lazy::new(Client::default);

fn get_cdn_url() -> Url {
    const GET_CUSTOM_CONTENT_URL: &str = "https://www.boomlings.com/database/getCustomContentURL.php";
    const FALLBACK_CDN_URL: &str = "https://geometrydashfiles.b-cdn.net";

    let url = CLIENT
        .post(GET_CUSTOM_CONTENT_URL)
        .header(USER_AGENT, "")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send().ok()
        .filter(|response| response.status().is_success())
        .and_then(|response| response.text().ok())
        .unwrap_or(String::from(FALLBACK_CDN_URL));

    Url::parse(&url).unwrap()
}

static SFX_URL_PATH: Lazy<Url> = Lazy::new(|| get_cdn_url().join("sfx/").unwrap());

pub(crate) fn fetch_library_version() -> Option<usize> {
    const SFX_VERSION_ENDPOINT: &str = "sfxlibrary_version.txt";

    let url = SFX_URL_PATH.join(SFX_VERSION_ENDPOINT).unwrap();
    CLIENT
        .get(url.as_str())
        .send().ok()?
        .text().ok()?
        .parse().ok()
}

pub(crate) fn fetch_library_data() -> Option<Bytes> {
    const SFX_LIBRARY_ENDPOINT: &str = "sfxlibrary.dat";

    let url = SFX_URL_PATH.join(SFX_LIBRARY_ENDPOINT).unwrap();
    CLIENT
        .get(url.as_str())
        .send().ok()?
        .bytes()
        .map(|bytes| bytes.to_vec()).ok()
}
