use std::io::prelude::*;
use base64::prelude::*;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

pub fn base64_decode(data: &[u8]) -> Vec<u8> {
    BASE64_URL_SAFE.decode(data).unwrap()
}

pub fn base64_encode(data: &[u8]) -> String {
    BASE64_URL_SAFE.encode(data)
}

pub fn zlib_decode(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len() * 2);
    let mut decoder = ZlibDecoder::new(data);
    decoder.read_to_end(&mut output).unwrap();
    output
}

pub fn zlib_encode(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len());
    let mut encoder = ZlibEncoder::new(&mut output, Compression::new(9));
    encoder.write_all(data).unwrap();
    drop(encoder);
    output
}
