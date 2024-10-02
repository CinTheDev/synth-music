use unnamed_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

pub mod drumset;

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

    pub fn generate(&self, info: ToneInfo<TET12ConcreteTone>) -> f32 {
        Self::triangle_wave(info) * self.decay_function(info) * info.intensity
    }

    fn triangle_wave(info: ToneInfo<TET12ConcreteTone>) -> f32 {
        use std::f64::consts::PI;
        let frequency = info.tone.to_frequency() as f64;
        let x = info.time.as_secs_f64() * frequency * 2.0 * PI;
        x.sin().asin() as f32
    }

    fn decay_function(&self, info: ToneInfo<TET12ConcreteTone>) -> f32 {
        0.5_f32.powf(info.time.as_secs_f32() * self.decay_speed)
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    pub fn generate(&self, info: ToneInfo<TET12ConcreteTone>) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, &info);
        }

        return amplitude * info.intensity;
    }

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, info: &ToneInfo<TET12ConcreteTone>) -> f32 {
        let factor = (2 * n + 1) as f32;
        let frequency = info.tone.to_frequency() as f64;
        Self::sine_wave(info.time, frequency * factor as f64) / factor
    }
}

impl Instrument for SoftBass {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: ToneInfo<Self::ConcreteValue>) -> f32 {
        self.generate(info)
    }
}

impl Instrument for HardBass {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: ToneInfo<Self::ConcreteValue>) -> f32 {
        self.generate(info)
    }
}
