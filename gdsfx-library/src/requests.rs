use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::{blocking::{Client, Response}, header::*};
use url::Url;

static CLIENT: Lazy<Client> = Lazy::new(Client::default);

fn get_cdn_url() -> Url {
    const CDN_URL_REQUEST_URL: &str = "https://www.boomlings.com/database/getCustomContentURL.php";
    const FALLBACK_CDN_URL: &str = "https://geometrydashfiles.b-cdn.net";

    let url = CLIENT
        .post(CDN_URL_REQUEST_URL)
        .header(USER_AGENT, "")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send().ok()
        .filter(|response| response.status().is_success())
        .and_then(|response| response.text().ok())
        .unwrap_or(FALLBACK_CDN_URL.to_string());

    Url::parse(&url).unwrap()
}

static SFX_URL_PATH: Lazy<Url> = Lazy::new(|| get_cdn_url().join("sfx").unwrap());

pub(crate) fn request_file(path: &str) -> Result<Response> {
    let response = CLIENT
        .get(SFX_URL_PATH.join(path).unwrap().as_str())
        .send()
        .and_then(|response| response.error_for_status())?;

    Ok(response)
}
