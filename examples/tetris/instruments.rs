use synth_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

pub mod drumset;
pub use drumset::{Drumset, DrumsetAction};

#[derive(Clone, Copy)]
pub struct SoftBass {
    pub decay_speed: f32,
}
    
#[derive(Clone, Copy)]
pub struct HardBass {
    pub harmonics: u32,
}

impl SoftBass {
    pub fn new(decay_speed: f32) -> Self {
        Self {
            decay_speed,
        }
    }

    fn generate(&self, tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += predefined::triangle_wave(frequency, time);
        }

        return result * self.decay_function(time);
    }

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

        return result * tones.intensity.start;
    }

    fn generate_frequency(&self, frequency: f64, time: Duration) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, frequency, time);
        }

        return amplitude;
    }

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, frequency: f64, time: Duration) -> f32 {
        let factor = (2 * n + 1) as f32;
        Self::sine_wave(time, frequency * factor as f64) / factor
    }
}

impl Instrument for SoftBass {
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
