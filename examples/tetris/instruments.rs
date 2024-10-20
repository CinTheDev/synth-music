use synth_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

pub mod drumset;
pub use drumset::{Drumset, DrumsetAction};

#[derive(Clone, Copy)]
pub struct Decaying<T: Instrument> {
    pub instrument: T,
    pub decay_speed: f32,
}
    
#[derive(Clone, Copy)]
pub struct HardBass {
    pub harmonics: u32,
}

impl<T: Instrument> Decaying<T> {
    fn decay_function(&self, time: Duration) -> f32 {
        0.5_f32.powf(time.as_secs_f32() * self.decay_speed)
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    fn generate(&self, tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += self.generate_frequency(frequency, time);
        }

        return result * Self::get_intensity(tones, time);
    }

    fn generate_frequency(&self, frequency: f64, time: Duration) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, frequency, time);
        }

        return amplitude;
    }

    fn harmonic(n: u32, frequency: f64, time: Duration) -> f32 {
        let factor = (2 * n + 1) as f32;
        let harmonic_frequency = frequency * factor as f64;
        predefined::sine_wave(harmonic_frequency, time) / factor.powf(1.7)
    }

    fn get_intensity(tones: &Tone<TET12ConcreteTone>, time: Duration) -> f32 {
        let beat_emphasis = tones.beat_emphasis.unwrap_or(1.0);

        let end_time = tones.play_duration.as_secs_f32();
        let time = time.as_secs_f32();
        let t = time / end_time;

        let intensity = t * (tones.intensity.end - tones.intensity.start) + tones.intensity.start;
        return intensity * beat_emphasis;
    }
}

impl<T: Instrument> Instrument for Decaying<T> {
    type ConcreteValue = T::ConcreteValue;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut instrument_buffer = self.instrument.render_buffer(buffer_info.clone(), tones);

        for i in 0..instrument_buffer.samples.len() {
            let time = buffer_info.time_from_index(i);
            let decay_factor = self.decay_function(time);
            instrument_buffer.samples[i] *= decay_factor;
        }

        return instrument_buffer;
    }
}

impl Instrument for HardBass {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(self.generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}
