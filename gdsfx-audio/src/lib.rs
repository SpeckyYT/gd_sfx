use std::{sync::Arc, thread};

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
    #[educe(Default = 0)]
    pub speed: i32,

    /// Semitone offset from `-12` to `12` (inclusive).
    #[educe(Default = 0)]
    pub pitch: i32,

    /// Volume range from `0.0` (no sound) to `2.0` (twice as loud).
    #[educe(Default = 1.0)]
    pub volume: f32,

    /// Whether to loop the sound.
    /// Takes into account start and end settings, as well as fading in once.
    #[educe(Default = false)]
    pub looping: bool,

    /// Start offset in milliseconds, before applying the speed modifier.
    #[educe(Default = 0)]
    pub start: u32,

    /// End point in milliseconds, before applying the speed modifier.
    /// The default value of 0 lets the sound play until it has finished.
    #[educe(Default = 0)]
    pub end: u32,

    /// Fade-in time in milliseconds, after applying the speed modifier.
    #[educe(Default = 0)]
    pub fade_in: u32,

    /// Fade-out time in milliseconds, after applying the speed modifier.
    #[educe(Default = 0)]
    pub fade_out: u32,
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

    // TODO: https://github.com/lebedec/libfmod-gen/issues/13
    pub fn play_file(&mut self, file_name: &str) -> Result<()> {
        self.stop_audio()?;

        let settings = *self.settings.read();

        let mut mode = Mode::DEFAULT;
        if settings.looping { mode |= Mode::LOOP_NORMAL };

        let sound = self.system.create_sound(file_name, mode, None)?;

        // Calculate start/end points
        let (sample_rate, _, _) = self.system.get_software_format()?;

        let sound_start = AudioSettings::millis_to_pcm(settings.start, sample_rate);
        let sound_end = match settings.end > 0 {
            true => AudioSettings::millis_to_pcm(settings.end, sample_rate),
            false => sound.get_length(TimeUnit::PCM)?,
        };

        // Prevent invalid parameters from being passed at all
        if sound_start >= sound_end { return Ok(()) }

        // Start/end points for looping sound
        sound.set_loop_points(sound_start, TimeUnit::PCM, sound_end, TimeUnit::PCM)?;

        let channel = self.system.play_sound(sound, None, false)?;
        let (_, start_point) = channel.get_dsp_clock()?;

        // Start offset and prepare fade in
        channel.set_position(sound_start, TimeUnit::PCM)?;
        channel.add_fade_point(start_point, 0.0)?;
        let fade_in_end = start_point + AudioSettings::millis_to_pcm(settings.fade_in, sample_rate) as u64;

        // Calculate end point and prepare fade out for single sound
        let end_point = start_point + (sound_end - sound_start) as u64;
        let fade_out_start = end_point - AudioSettings::millis_to_pcm(settings.fade_out, sample_rate) as u64;
        if !settings.looping {
            channel.add_fade_point(end_point, 0.0)?;
        }

        // Set up pitch shift
        let pitch_shift = self.system.create_dsp_by_type(DspType::Pitchshift)?;
        channel.add_dsp(ChannelControlDspIndex::Tail.into(), pitch_shift)?;

        *self.channel.write() = Some(channel);

        let channel = Arc::clone(&self.channel);
        let settings = Arc::clone(&self.settings);

        thread::spawn(move || -> Result<()> {
            loop {
                let channel = channel.read();

                // Channel was removed
                let Some(channel) = channel.as_ref() else { break };

                // Channel has finished
                if !channel.is_playing()? { break }

                // Stop if past end point
                if channel.get_position(TimeUnit::PCM)? >= sound_end {
                    channel.stop()?; // TODO rewrite using channels, then send a stop command
                    break
                }

                let settings = *settings.read();
    
                // If looping is disabled, let the current iteration finish
                if !settings.looping { channel.set_loop_count(0)? }

                channel.set_volume(settings.volume)?;

                // Update pitch shift
                let pitch = AudioSettings::linear_to_exp(settings.pitch);
                pitch_shift.set_parameter_float(DspPitchShift::Pitch.into(), pitch)?;

                // This pitch-setting function also stretches time
                channel.set_pitch(AudioSettings::linear_to_exp(settings.speed))?;

                // Update fade in/out points with actual volume                
                channel.add_fade_point(fade_in_end, settings.volume)?;
                if !mode.contains(Mode::LOOP_NORMAL) { // Initially looping sounds shouldn't fade out
                    channel.add_fade_point(fade_out_start, settings.volume)?;
                    // TODO: figure out stupid fade point placement with dsp clock and pcm fucky wucky
                    // let (dsp_clock, parent_clock) = channel.get_dsp_clock()?;
                    // let position = channel.get_position(TimeUnit::PCM)?;
                    // eprintln!("Start: {start_point} → {fade_in_end} (PCM {sound_start}) | End: {fade_out_start} → {end_point} (PCM {sound_end}) | Pos: {dsp_clock}/{parent_clock} (PCM {position})");
                    // thread::sleep_ms(8);
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

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new().expect("Couldn't initialize audio system")
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
