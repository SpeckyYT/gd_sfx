use std::io::Cursor;

use rodio::{OutputStream, Sink, Decoder};

pub fn play_ogg(ogg: Vec<u8>) {
    let cursor = Cursor::new(ogg);
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    sink.append(Decoder::new(cursor).unwrap());
    sink.sleep_until_end();
}
