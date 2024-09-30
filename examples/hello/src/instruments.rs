use unnamed_music::melody::prelude::*;
use unnamed_music::melody::instrument::predefined::PredefinedInstrument;

pub struct HarmonicWave {
    count: u32,
}

pub enum Instruments {
    Predefined(PredefinedInstrument),
    HarmonicWave(HarmonicWave),
}

impl Instruments {
    pub fn new_harmonic_wave(count: u32) -> Self {
        Self::HarmonicWave(HarmonicWave {
            count,
        })
    }

    fn predefined(instrument: &PredefinedInstrument, info: ToneInfo) -> f32 {
        instrument.generate_sound(info)
    }

    fn harmonic_wave(attributes: &HarmonicWave, info: ToneInfo) -> f32 {
        let base_frequency = info.tone.to_frequency();
        let seconds = info.time.as_secs_f64();

        let mut result = 0.0;

        for i in 0..attributes.count {
            let frequency = base_frequency as f64 * i as f64;
            result += Self::sine_wave(frequency, seconds) * info.intensity * Self::intensity_factor(i);
        }

        return result;
    }

    fn intensity_factor(n: u32) -> f32 {
        0.5_f32.powi(n as i32)
    }

    fn sine_wave(frequency: f64, seconds: f64) -> f32 {
        use std::f64::consts::PI;
        (seconds * frequency * 2.0 * PI).sin() as f32
    }
}

impl Instrument for Instruments {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        match self {
            Self::Predefined(instrument) => Self::predefined(instrument, info),
            Self::HarmonicWave(attributes) => Self::harmonic_wave(attributes, info),
        }
    }
}
