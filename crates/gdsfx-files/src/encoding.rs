use std::io::prelude::*;

use base64::prelude::*;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

pub fn base64_decode(bytes: &[u8]) -> Vec<u8> {
    BASE64_URL_SAFE.decode(bytes).unwrap()
}

pub fn base64_encode(bytes: &[u8]) -> String {
    BASE64_URL_SAFE.encode(bytes)
}

pub fn zlib_decode(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len() * 2);
    let mut decoder = ZlibDecoder::new(bytes);
    decoder.read_to_end(&mut output).unwrap();
    output
}

pub fn zlib_encode(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut encoder = ZlibEncoder::new(&mut output, Compression::new(9));
    encoder.write_all(bytes).unwrap();
    drop(encoder); // why? // bro, try removing it and https://tryitands.ee
    output
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let bytes = base64_decode(bytes);
    zlib_decode(&bytes)
}

pub fn encode(bytes: &[u8]) -> String {
    let bytes = zlib_encode(bytes);
    base64_encode(&bytes)
}
