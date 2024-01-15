use std::{
    io::Cursor,
    sync::Arc,
    time::Instant,
};

use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rodio::{Decoder, OutputStream, Sink};
use kittyaudio::{Frame, Mixer, interpolate_frame, Sound};

#[derive(Debug, Clone, Copy)]
pub struct AudioSettings {
    pub volume: f32, // 0..=2
    pub speed: f32, // -12..=12
    pub pitch: f32, // -12..=12
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

    if settings.pitch != 0.0 || settings.speed != 1.0 {
        let pitch_correction = 2_f32.powf(settings.pitch / 12.0);
        let speed_correction = 1.0 / 2_f32.powf(settings.speed / 12.0);
        let pitch_correction = pitch_correction / speed_correction;

        let mut mixer = Mixer::new();
        mixer.init();

        // println!("{:?}", bot::AudioSegment::from_bytes(ogg.clone()));

        if let Ok(mut audio_segment) = bot::AudioSegment::from_bytes(ogg) {
            let initial_sample_rate = audio_segment.sample_rate;
            let new_rate = (initial_sample_rate as f32 * pitch_correction) as u32;

            audio_segment.resample(new_rate);
            audio_segment.set_volume(settings.volume);

            let frames = audio_segment.frames.iter().map(|frame| {
                Frame::new(frame.left, frame.right)
            })
            .collect::<Vec<Frame>>();

            let sound = Sound::from_frames(initial_sample_rate, &frames);

            let sound_handle = mixer.play(sound.clone());

            while !mixer.is_finished() {
                if let Ok(received_time) = AUDIO_MESSAGES.receiver.try_recv() {
                    if received_time > start_time {
                        sound_handle.pause();
                        break;
                    }
                }
            }
        }
    } else {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        let sfx_data = Decoder::new(Cursor::new(ogg)).unwrap();
        sink.set_volume(settings.volume);
        sink.append(sfx_data);

        while !sink.empty() {
            if let Ok(received_time) = AUDIO_MESSAGES.receiver.try_recv() {
                if received_time > start_time {
                    sink.stop();
                    break;
                }
            }
        }
    }

    *PLAYERS.lock() -= 1;
}

pub fn is_playing_audio() -> bool {
    *PLAYERS.lock() > 0
}

pub fn stop_all() {
    for _ in 0..*PLAYERS.lock() {
        AUDIO_MESSAGES.sender.send(Instant::now()).unwrap();
    }
}

#[derive(Clone, Debug, Default)]
pub struct AudioSegment {
    pub sample_rate: u32,
    /// Interleaved channel data. Always [`AudioSegment::NUM_CHANNELS`] channels.
    pub frames: Vec<Frame>,
    pub pitch_table: Vec<AudioSegment>,
}

impl AudioSegment {
    pub fn resample(&mut self, rate: u32) -> &mut Self {
        let mut fractional_position = 0.0f64;
        let mut iter = self.frames.iter();
        let mut frames = [Frame::ZERO; 4]; // prev, cur, next, next next
        macro_rules! push_frame {
            ($frame:expr) => {
                for i in 0..frames.len() - 1 {
                    frames[i] = frames[i + 1];
                }
                frames[frames.len() - 1] = $frame;
            };
        }

        // fill resampler with 3 frames
        for _ in 0..3 {
            push_frame!(iter.next().copied().unwrap_or(Frame::ZERO));
        }

        let mut resampled_frames = Vec::with_capacity(self.frames.len());
        let dt = rate as f64 / self.sample_rate as f64;

        'outer: loop {
            resampled_frames.push(interpolate_frame(
                frames[0],
                frames[1],
                frames[2],
                frames[3],
                fractional_position as f32,
            ));

            fractional_position += dt;
            while fractional_position >= 1.0 {
                fractional_position -= 1.0;
                let Some(frame) = iter.next().copied() else {
                    break 'outer;
                };
                push_frame!(frame);
            }
        }

        self.sample_rate = rate;
        self.frames = resampled_frames;
        self
    }
}
