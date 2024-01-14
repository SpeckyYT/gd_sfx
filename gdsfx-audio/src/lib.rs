use std::{
    io::Cursor,
    sync::Arc,
    time::Instant,
};

use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Debug, Clone, Copy)]
pub struct AudioSettings {
    volume: f32,
    speed: f32,
    pitch: f32, // -12..12
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            speed: 1.0,
            pitch: 0.0,
        }
    }
}

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

pub fn play_sound(ogg: Vec<u8>, settings: AudioSettings) {
    *PLAYERS.lock() += 1;
    let start_time = Instant::now();

    // i have no idea what this does so im just gonna leave it
    // ok zoomer

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let sfx_data = Decoder::new(Cursor::new(ogg)).unwrap();

    sink.set_volume(settings.volume);

    if settings.pitch != 0.0 || settings.speed != 1.0 {
        let pitch_correction = 2f32.powf(settings.pitch / 12.0);
        let pitch_correction = pitch_correction / settings.speed;

        println!("TODO: pitch should be corrected by {}", pitch_correction);

        sink.set_speed(settings.speed);
    }

    sink.append(sfx_data);

    while !sink.empty() {
        if let Ok(received_time) = AUDIO_MESSAGES.receiver.try_recv() {
            if received_time > start_time {
                sink.stop();
            }
        }
    }

    *PLAYERS.lock() -= 1;
}

pub fn stop_all() {
    for _ in 0..*PLAYERS.lock() {
        AUDIO_MESSAGES.sender.send(Instant::now()).unwrap();
    }
}
