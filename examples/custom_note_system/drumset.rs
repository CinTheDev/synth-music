use synth_music::prelude::*;
use std::time::Duration;

// Specify possible drumset actions
#[derive(Clone, Copy)]
pub enum DrumsetAction {
    Bass,
    Snare,
    HiHat,
}

// !! This is important
// This will convert the ScaledValue into a ConcreteValue, which is then
// given to the instrument. It's also possible to not perform any conversion
// at all and to just return a copy of self. (But this still needs to be
// implemented to be placed on tracks.)
impl ScaledValue for DrumsetAction {
    type ConcreteValue = Self;

    // The music key is availabe for conversion, but in this example it is not needed.
    fn to_concrete_value(&self, _key: MusicKey) -> Self::ConcreteValue {
        self.to_owned()
    }
}

#[derive(Clone, Copy)]
pub struct Drumset {
    bass_duration: Duration,
    snare_duration: Duration,
    hihat_duration: Duration,
}

impl Drumset {
    pub fn new() -> Self {
        Self {
            bass_duration: Duration::from_secs_f32(0.4),
            snare_duration: Duration::from_secs_f32(0.3),
            hihat_duration: Duration::from_secs_f32(0.15),
        }
    }

    /// The bass is a low-frequency sine wave. The frequency starts higher, and
    /// quickly goes really low.
    pub fn bass(&self, buffer: &mut SoundBuffer) {
        let sample_rate = buffer.settings().sample_rate;
        let num_samples = Self::get_target_samples(sample_rate, self.bass_duration);
        buffer.samples = vec![0.0; num_samples];

        for i in 0..num_samples {
            let time = buffer.time_from_index(i);
            let frequency = self.bass_frequency(time) as f64;
            let value = predefined::sine_wave(frequency, time);
            buffer.samples[i] = value * self.decay(time, self.bass_duration);
        }
    }

    fn bass_frequency(&self, time: Duration) -> f32 {
        let start_frequency = 90.0;
        let end_frequency = 55.0;

        let max_time = self.bass_duration;
        let t = time.as_secs_f32() / max_time.as_secs_f32();

        return Self::interpolation_exponential(start_frequency, end_frequency, t);
    }

    fn interpolation_exponential(a: f32, b: f32, t: f32) -> f32 {
        use std::f32::consts::E;
        let speed = -4.0;
        let e_speed = E.powf(speed);
        let offset = e_speed / (e_speed - 1.0);
        let coef = 1.0 - offset;

        let t_factor = coef * E.powf(speed * t) + offset;

        return t_factor * (a - b) + b;
    }

    /// Snares and HiHats are white noise with a frequency filter applied.
    /// Snares have a broad range of middle frequencies, HiHats are higher
    /// frequencies.
    pub fn noised_tone(&self, buffer: &mut SoundBuffer, action: &DrumsetAction) {
        let frequency_range = match action {
            DrumsetAction::Snare => 500.0 .. 11000.0,
            DrumsetAction::HiHat => 6000.0 .. 20000.0,
            
            _ => panic!("Invalid action in noised_tone"),
        };
        let target_duration = match action {
            DrumsetAction::Snare => self.snare_duration,
            DrumsetAction::HiHat => self.hihat_duration,

            _ => panic!("Invalid action in noised_tone"),
        };

        let sample_rate = buffer.settings().sample_rate;
        let num_samples = Self::get_target_samples(sample_rate, target_duration);

        buffer.samples = vec![0.0; num_samples];
        noise::white_noise(&mut buffer.samples);
        eq::filter_fft_whole(buffer, |f: f32| -> f32 {
            let leakage_distance = 5000.0;
            if f < frequency_range.start {
                let dist = frequency_range.start - f;
                let t = (dist / leakage_distance).min(1.0);
                return Self::lerp(1.0, 0.0, t);
            }
            if f > frequency_range.end {
                let dist = f - frequency_range.end;
                let t = (dist / leakage_distance).min(1.0);
                return Self::lerp(1.0, 0.0, t);
            }

            return 1.0;
        });

        for i in 0..buffer.samples.len() {
            let time = buffer.time_from_index(i);
            buffer.samples[i] *= self.decay(time, target_duration) * 0.4;
        }
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        return t * (b - a) + a;
    }

    fn decay(&self, time: Duration, target_duration: Duration) -> f32 {
        let factor = 5.0 / target_duration.as_secs_f32();
        0.5_f32.powf(time.as_secs_f32() * factor)
    }

    fn get_target_samples(sample_rate: u32, target_duration: Duration) -> usize {
        (target_duration.as_secs_f32() * sample_rate as f32).ceil() as usize
    }
}

impl Instrument for Drumset {
    type ConcreteValue = DrumsetAction;

    fn render_tone_buffer(
        &self,
        tone: Self::ConcreteValue,
        buffer: &mut SoundBuffer,
        _num_samples: usize,
    ) {
        match tone {
            DrumsetAction::Bass => {
                self.bass(buffer);
            },

            _ => {
                self.noised_tone(buffer, &tone);
            }
        }
    }
}
