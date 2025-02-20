use image::{imageops::FilterType, ImageReader};
use std::io::Cursor;

const ICON: &[u8] = include_bytes!(gdsfx_files::workspace_path!("assets/normal.png"));

pub fn build() {
    gdsfx_files::build::write_output_bytes("icon.bin", load_image_bytes(ICON));
}

fn load_image_bytes(bytes: &[u8]) -> Vec<u8> {
    ImageReader::new(Cursor::new(bytes))
        .with_guessed_format().unwrap()
        .decode().unwrap()
        .resize(gdsfx_files::consts::ICON_SIZE, gdsfx_files::consts::ICON_SIZE, FilterType::Triangle)
        .to_rgba8()
        .to_vec()
}
