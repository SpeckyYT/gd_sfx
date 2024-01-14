const NORMAL_OUTPUT_FILE: &str = "normal.bin";

use image::{io::Reader, imageops::FilterType, DynamicImage};
use std::io::Cursor;

pub fn build() {
    let normal = load_and_resize(include_bytes!("../../assets/normal.png"));

    gdsfx_build::write_output_bytes(NORMAL_OUTPUT_FILE, normal.to_rgba8().to_vec());
}

fn load_and_resize(bytes: &[u8]) -> DynamicImage {
    let img = Reader::new(Cursor::new(bytes))
    .with_guessed_format()
    .unwrap()
    .decode()
    .unwrap();

    img.resize(256, 256, FilterType::Triangle)
}
