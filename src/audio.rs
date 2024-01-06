use std::{
    io::Cursor,
    thread::{JoinHandle, self},
    time::Instant,
    sync::Arc
};

use crossbeam_channel::{unbounded, Sender, Receiver};
use eframe::epaint::mutex::Mutex;
use lazy_static::lazy_static;
use rodio::{OutputStream, Sink, Decoder};

use crate::library::LibraryEntry;

lazy_static!{
    pub static ref PLAYERS: Arc<Mutex<usize>> = Default::default();
    pub static ref AUDIO_MESSAGES: (Sender<Instant>, Receiver<Instant>) = unbounded();
}

pub fn play_sound(sfx: &LibraryEntry) {
    let data = sfx.download();
    if let Some(content) = data {
        play_ogg(content);
    }
}

pub fn play_ogg(ogg: Vec<u8>) -> JoinHandle<()> {
    thread::spawn(|| {
        *PLAYERS.lock() += 1;
        let start_time = Instant::now();
        let cursor = Cursor::new(ogg);
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        sink.append(Decoder::new(cursor).unwrap());
        while !sink.empty() {
            if let Ok(received_time) = AUDIO_MESSAGES.1.try_recv() {
                if received_time > start_time {
                    sink.stop();
                }
            }
        }
        *PLAYERS.lock() -= 1;
    })
}

pub fn stop_audio() {
    for _ in 0..*PLAYERS.lock() {
        AUDIO_MESSAGES.0.send(Instant::now()).unwrap();
    }
}
