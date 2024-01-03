use std::io::prelude::*;
use base64::prelude::*;
use flate2::read::ZlibDecoder;

pub fn base64_decode(data: &[u8]) -> Vec<u8> {
    BASE64_URL_SAFE.decode(data).unwrap()
}

pub fn zlib_decoder(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len() * 2);
    let mut decoder = ZlibDecoder::new(data);
    decoder.read_to_end(&mut output).unwrap();
    output
}
