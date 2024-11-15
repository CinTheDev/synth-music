///
/// This implementation has been copied from the "custom_note_system" example.
/// Check that example for details.
/// 

use synth_music::prelude::*;
use std::time::Duration;
use rand::Rng;

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
    pub fn bass(&self, buffer_info: &BufferInfo) -> Vec<f32> {
        let target_samples = Self::get_target_samples(buffer_info.sample_rate, self.bass_duration);

        let mut buffer = vec![0_f32; target_samples];

        for i in 0..buffer.len() {
            let time = buffer_info.time_from_index(i);
            let frequency = self.bass_frequency(time) as f64;
            let value = predefined::sine_wave(frequency, time);
            buffer[i] = value * self.decay(time, self.bass_duration);
        }

        return buffer;
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
    pub fn noised_tone(&self, buffer_info: &BufferInfo, action: &DrumsetAction) -> Vec<f32> {
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

        let target_samples = Self::get_target_samples(buffer_info.sample_rate, target_duration);
        let mut buffer = vec![0_f32; target_samples];
        
        Self::generate_white_noise(&mut buffer);
        Self::filter(&mut buffer, buffer_info, frequency_range);

        for i in 0..buffer.len() {
            let time = buffer_info.time_from_index(i);
            buffer[i] *= self.decay(time, target_duration) * 0.2;
        }

        return buffer;
    }

    pub fn generate_white_noise(buffer: &mut Vec<f32>) {
        let mut rng = rand::thread_rng();

        for sample in buffer.iter_mut() {
            *sample = rng.gen_range(-1.0..1.0);
        }
    }

    pub fn filter(buffer: &mut Vec<f32>, buffer_info: &BufferInfo, frequency: std::ops::Range<f32>) {
        use biquad::*;

        let f_lower = frequency.start.hz();
        let f_upper = frequency.end.hz();
        let f_sample = buffer_info.sample_rate.hz();

        let coeffs_lp = Coefficients::<f32>::from_params(
            Type::LowPass,
            f_sample,
            f_upper,
            Q_BUTTERWORTH_F32,
        ).unwrap();
        let coeffs_hp = Coefficients::<f32>::from_params(
            Type::HighPass,
            f_sample,
            f_lower,
            Q_BUTTERWORTH_F32,
        ).unwrap();

        let mut biquad_lp = DirectForm1::<f32>::new(coeffs_lp);
        let mut biquad_hp = DirectForm1::<f32>::new(coeffs_hp);
        
        for sample in buffer.iter_mut() {
            *sample = biquad_lp.run(*sample);
            *sample = biquad_hp.run(*sample);
        }
    }

    fn mix_buffers(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
        let (mut larger, smaller) = match a.len() >= b.len() {
            true => (a, b),
            false => (b, a),
        };

        for i in 0..smaller.len() {
            larger[i] += smaller[i];
        }

        return larger;
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

    fn render(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut result = vec![0.0; buffer_info.tone_samples];
        
        for action in &tones.concrete_values {
            match action {
                DrumsetAction::Bass => {
                    let buffer = self.bass(&buffer_info);
                    result = Self::mix_buffers(result, buffer);
                },

                _ => {
                    let buffer = self.noised_tone(&buffer_info, action);
                    result = Self::mix_buffers(result, buffer);
                }
            }
        }

        let intensity = tones.intensity.start * tones.beat_emphasis.unwrap_or(1.0);

        for value in result.iter_mut() {
            *value *= intensity;
        }

        InstrumentBuffer { samples: result }
    }
}
