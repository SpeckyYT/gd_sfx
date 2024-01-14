use image::{io::Reader, imageops::FilterType};
use std::io::Cursor;

const OUTPUT_FILE: &str = "normal.bin";

pub fn build() {
    let bytes = load_image_bytes(include_bytes!("../../assets/normal.png"));
    gdsfx_build::write_output_bytes(OUTPUT_FILE, bytes);
}

fn load_image_bytes(bytes: &[u8]) -> Vec<u8> {
    Reader::new(Cursor::new(bytes))
        .with_guessed_format().unwrap()
        .decode().unwrap()
        .resize(256, 256, FilterType::Triangle)
        .to_rgba8()
        .to_vec()
}
