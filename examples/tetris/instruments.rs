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
}

impl<T: Instrument> Instrument for Decaying<T> {
    type ConcreteValue = T::ConcreteValue;

    fn render(&self, tones: &Tone<Self::ConcreteValue>, buffer: &mut SoundBuffer) {
        self.instrument.render(tones, buffer);

        for i in 0..buffer.samples.len() {
            let time = buffer.time_from_index(i);
            let decay_factor = self.decay_function(time);
            buffer.samples[i] *= decay_factor;
        }
    }
}

impl Instrument for HardBass {
    type ConcreteValue = TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return self.generate_frequency(frequency, time);
    }
}
