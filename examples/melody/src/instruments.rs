use unnamed_music::melody::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct SoftBass {
    decay_speed: f32,
}

#[derive(Clone, Copy)]
pub struct HardBass {
    harmonics: u32,
}

impl SoftBass {
    pub fn new(decay_speed: f32) -> Self {
        Self {
            decay_speed,
        }
    }

    fn triangle_wave(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        let x = info.time.as_secs_f64() * info.frequency * 2.0 * PI;
        x.sin().asin() as f32
    }

    fn decay_function(&self, info: ToneInfo) -> f32 {
        0.5_f32.powf(info.time.as_secs_f32() * self.decay_speed)
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, info: &ToneInfo) -> f32 {
        let factor = (2 * n + 1) as f32;
        Self::sine_wave(info.time, info.frequency * factor as f64) / factor
    }
}

impl Instrument for SoftBass {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        Self::triangle_wave(info) * self.decay_function(info)
    }
}

impl Instrument for HardBass {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, &info);
        }

        return amplitude;
    }
}
