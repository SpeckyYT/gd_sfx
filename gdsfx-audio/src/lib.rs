use std::{
    io::Cursor,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Instant,
};

use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rodio::{Decoder, OutputStream, Sink};

// pub visibility required by lazy_static
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> From<(Sender<T>, Receiver<T>)> for Channel<T> {
    fn from((sender, receiver): (Sender<T>, Receiver<T>)) -> Self {
        Self { sender, receiver }
    }
}

lazy_static! {
    static ref PLAYERS: Arc<Mutex<usize>> = Default::default();
    static ref AUDIO_MESSAGES: Channel<Instant> = crossbeam_channel::unbounded().into();
}

pub fn play_sound(ogg: Vec<u8>) -> JoinHandle<()> {
    thread::spawn(|| {
        // i have no idea what this does so im just gonna leave it
        *PLAYERS.lock() += 1;
        let start_time = Instant::now();
        let cursor = Cursor::new(ogg);
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        sink.append(Decoder::new(cursor).unwrap());
        while !sink.empty() {
            if let Ok(received_time) = AUDIO_MESSAGES.receiver.try_recv() {
                if received_time > start_time {
                    sink.stop();
                }
            }
        }
        *PLAYERS.lock() -= 1;
    })
}

pub fn stop_all() {
    for _ in 0..*PLAYERS.lock() {
        AUDIO_MESSAGES.sender.send(Instant::now()).unwrap();
    }
}
