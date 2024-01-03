use std::fs;
use std::{env, path::PathBuf};

use reqwest::blocking::Client;
use reqwest::header::{USER_AGENT, CONTENT_TYPE};

use base64::prelude::*;

const ENDPOINT_SFX_VERSION: &str = "sfx/sfxlibrary_version.txt";
const ENDPOINT_SFX_LIBRARY: &str = "sfx/sfxlibrary.dat";
const XOR_KEY_SFX_DATA: u16 = 48291;

fn main() {
    let local_appdata_folder = PathBuf::from(env::var("localappdata").unwrap());
    let gd_folder = local_appdata_folder.join("GeometryDash");

    let client = Client::builder().build().unwrap();

    let request = client
    .post("https://www.boomlings.com/database/getCustomContentURL.php")
    .header(USER_AGENT, "")
    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    .send()
    .unwrap();

    let cdn_url = request.text().unwrap();
    
    println!("CDN URL: {}", cdn_url);

    let sfx_version = client.get(format!("{cdn_url}/{ENDPOINT_SFX_VERSION}")).send().unwrap().text().unwrap();

    println!("SFX VERSION: {}", sfx_version);

    let sfx_data = client.get(format!("{cdn_url}/{ENDPOINT_SFX_LIBRARY}")).send().unwrap().text().unwrap();

    let sfx_data_decoded = decode_base64(format!("{}==", sfx_data).as_bytes());
    
    // let xor_cryptor = xor_cryptor::XORCryptor::new(&char::from_u32(XOR_KEY_SFX_DATA).unwrap().to_string()).unwrap();

    // let deciphered = xor_cryptor(XOR_KEY_SFX_DATA.to_string().as_bytes(), &sfx_data_decoded);

    // println!("{:?}", deciphered);

    for i in 170..14260 {
        let filename = format!("s{i}.ogg");
        let filepath = gd_folder.join(&filename);
        let file_url = format!("{cdn_url}/sfx/{filename}");

        if filepath.exists() { continue }

        let request = client.get(file_url).send().unwrap();

        if request.status().is_success() {
            let data = request.bytes().unwrap();

            fs::write(filepath, data).unwrap();

            println!("[SUCCESS] {filename}");
        } else {
            println!("[FAILED] {filename}");
        }

        // let a = get();
    }
}

fn decode_base64(data: &[u8]) -> Vec<u8> {
    let mut decoded_data = Vec::new();
    let mut buffer = 0u32;
    let mut buffer_size = 0usize;

    for byte in data {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => continue,
        } as u32;

        buffer = (buffer << 6) | value;
        buffer_size += 6;

        if buffer_size >= 8 {
            buffer_size -= 8;
            decoded_data.push((buffer >> buffer_size) as u8);
            buffer &= (1 << buffer_size) - 1;
        }
    }

    decoded_data
}

fn xor_cryptor(key: &[u8], data: &[u8]) -> String {
    let mut result = String::new();
    for i in 0..data.len() {
        result.push((data[i] ^ key[i % key.len()]) as char);
    }
    result
}
