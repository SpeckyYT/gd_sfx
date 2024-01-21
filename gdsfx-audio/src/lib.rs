use std::{sync::Arc, thread, mem};

use educe::Educe;
use libfmod::*;
use parking_lot::RwLock;

type Result<T> = std::result::Result<T, libfmod::Error>;

pub struct AudioSystem {
    /// The FMOD system object to be used internally.
    system: System,

    /// The FMOD channel which is currently playing sound, or `None` if no sound is being played.
    channel: Arc<RwLock<Option<Channel>>>,

    /// Public instance of `AudioSettings` that can be modified at any time.
    /// 
    /// Only specific changes to this struct can be applied immediately while playing audio though;
    /// see the fields of the `AudioSettings` struct for more information.
    pub settings: Arc<RwLock<AudioSettings>>,
}

#[derive(Educe, Debug, Clone, Copy, PartialEq)]
#[educe(Default)]
pub struct AudioSettings {
    /// Exponential speed factor scale from `-12` to `12` (inclusive).
    /// `-12` represents half speed, while `-12` is double speed.
    /// 
    /// Also distorts pitch by this many semitones.
    /// 
    /// Modifiable while playing.
    #[educe(Default = 0)]
    pub speed: i32,

    /// Semitone offset from `-12` to `12` (inclusive).
    #[educe(Default = 0)]
    pub pitch: i32,

    /// Volume range from `0.0` (no sound) to `2.0` (twice as loud).
    /// 
    /// Modifiable while playing.
    #[educe(Default = 1.0)]
    pub volume: f32,

    /// Whether to loop the sound.
    /// Takes into account start and end settings, as well as fading in once.
    /// 
    /// Can be set to false while playing to stop looping.
    #[educe(Default = false)]
    pub looped: bool,

    /// Start offset in milliseconds, before applying the speed modifier.
    #[educe(Default = 0)]
    pub start: u32,

    /// End point in milliseconds, before applying the speed modifier.
    #[educe(Default = 0)]
    pub end: u32,

    /// Fade-in time in milliseconds, after applying the speed modifier.
    #[educe(Default = 0)]
    pub fade_in: u32,

    /// Fade-out time in milliseconds, after applying the speed modifier.
    #[educe(Default = 0)]
    pub fade_out: u32,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl AudioSystem {
    pub fn new() -> Result<Self> {
        let system = System::create()?;
    
        // Since previously playing audio is stopped when new audio should be played,
        // a maximum of 1 channel should be enough.
        system.init(1, Init::NORMAL, None)?;

        Ok(Self {
            system,
            channel: Default::default(),
            settings: Default::default(),
        })
    }

    pub fn play_sound(&mut self, bytes: impl AsRef<[u8]>) -> Result<()> {
        self.stop_audio()?;

        let settings = *self.settings.read();

        let mode = if settings.looped { Mode::LOOP_NORMAL } else { Mode::DEFAULT };

        // SAFETY: System::create_sound requires the first parameter to be a &str,
        // which is immediately converted back to a Vec<u8> in the CString constructor though.
        let sound = unsafe {
            #[allow(clippy::transmute_bytes_to_str)] // these bytes might not be valid UTF-8
            let bytes = mem::transmute(bytes.as_ref());
            self.system.create_sound(bytes, mode, None)?
        };

        // Calculate start/end points
        // TODO: figure out points with speed factor
        let (sample_rate, _, _) = self.system.get_software_format()?;

        let start_point = AudioSettings::millis_to_pcm(settings.start, sample_rate);

        let end_point = match settings.end > 0 {
            true => AudioSettings::millis_to_pcm(settings.end, sample_rate),
            false => sound.get_length(TimeUnit::PCM)?,
        };

        // Prevent invalid parameters from being passed at all
        if settings.volume == 0.0 || start_point >= end_point { return Ok(()) }

        // Start/end points for looped sound
        if settings.looped {
            sound.set_loop_points(start_point, TimeUnit::PCM, end_point, TimeUnit::PCM)?;
        }

        let channel = self.system.play_sound(sound, None, false)?;

        // Prepare start offset and fade in
        // TODO: figure out overlapping fade times, and speed factor
        channel.set_position(start_point, TimeUnit::PCM)?;
        channel.add_fade_point(0, 0.0)?;

        let fade_in_time = AudioSettings::millis_to_pcm(settings.fade_in, sample_rate) as u64;

        // Prepare end point and fade out for single sound
        // TODO: figure out overlapping fade times, and speed factor
        let duration = (end_point - start_point) as u64;
        if !settings.looped {
            channel.add_fade_point(duration, 0.0)?;
            channel.set_delay(None, Some(duration), true)?;
        }

        let fade_out_time = AudioSettings::millis_to_pcm(settings.fade_out, sample_rate) as u64;

        // Pitch shift
        let pitch = AudioSettings::linear_to_exp(settings.pitch);
        let pitch_shift = self.system.create_dsp_by_type(DspType::Pitchshift)?;
        pitch_shift.set_parameter_float(DspPitchShift::Pitch.into(), pitch)?;
        channel.add_dsp(ChannelControlDspIndex::Tail.into(), pitch_shift)?;

        *self.channel.write() = Some(channel);

        let channel = self.channel.clone();
        let settings = self.settings.clone();

        thread::spawn(move || -> Result<()> {
            loop {
                let Some(channel) = *channel.read() else { break };
                if !channel.is_playing()? { break }

                let settings = *settings.read();

                if !settings.looped {
                    // If looping is disabled, let the current iteration finish
                    channel.set_loop_count(0)?;
                }

                channel.set_volume(settings.volume)?;

                // This pitch-setting function also stretches time
                channel.set_pitch(AudioSettings::linear_to_exp(settings.speed))?;

                // Update fade in/out points with actual volume
                channel.add_fade_point(fade_in_time, settings.volume)?;
                if mode != Mode::LOOP_NORMAL { // Initially looped sounds shouldn't fade out
                    channel.add_fade_point(duration - fade_out_time, settings.volume)?;
                }
            }

            Ok(())
        });

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.channel.read()
            .and_then(|channel| channel.is_playing().ok())
            .unwrap_or(false)
    }

    pub fn stop_audio(&mut self) -> Result<()> {
        match self.channel.write().take() {
            Some(channel) => channel.stop(),
            None => Ok(()),
        }
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        let _ = self.system.release();
    }
}

impl AudioSettings {
    fn linear_to_exp(num: i32) -> f32 {
        2.0_f32.powf(num as f32 / 12.0)
    }

    fn millis_to_pcm(millis: u32, sample_rate: i32) -> u32 {
        (millis as f32 / 1000.0 * sample_rate.unsigned_abs() as f32) as u32
    }
}
