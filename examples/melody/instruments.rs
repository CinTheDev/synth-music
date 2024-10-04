use synth_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

pub mod drumset;
use drumset::DrumsetAction;

#[derive(Clone, Copy)]
pub struct SoftBass {
    pub decay_speed: f32,
}
    
#[derive(Clone, Copy)]
pub struct HardBass {
    pub harmonics: u32,
}

#[derive(Clone, Copy)]
pub struct Drumset {

}

impl SoftBass {
    pub fn new(decay_speed: f32) -> Self {
        Self {
            decay_speed,
        }
    }

    pub fn generate(&self, frequency: f64, time: Duration) -> f32 {
        Self::triangle_wave(frequency, time) * self.decay_function(time)
    }

    fn triangle_wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        let x = time.as_secs_f64() * frequency * 2.0 * PI;
        x.sin().asin() as f32
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

    pub fn generate(&self, frequency: f64, time: Duration) -> f32 {
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

// TODO: Improve this (make the DrumsetAction actually matter)
impl Drumset {
    fn random() -> f32 {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        rng.gen_range(-1.0..1.0)
    }

    fn frequency_range(action: DrumsetAction) -> (f32, f32) {
        match action {
            DrumsetAction::Bass => (0.0, 100.0),
            DrumsetAction::Snare => (100.0, 200.0),
            DrumsetAction::HiHat => (1000.0, 2000.0),
        }
    }

    fn generate(&self, time: Duration) -> f32 {
        if time > Duration::from_millis(100) {
            return 0.0;
        }

        let value = Self::random();
        return value;
    }

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Instrument for SoftBass {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += self.generate(frequency, time);
        }

        return result * info.intensity.start;
    }
}

impl Instrument for HardBass {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += self.generate(frequency, time);
        }

        return result * info.intensity.start;
    }
}

impl Instrument for Drumset {
    type ConcreteValue = DrumsetAction;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        self.generate(time) * info.intensity.start
    }
}
