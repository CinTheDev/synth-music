pub mod tet12;
use tet12::TET12ConcreteTone;

use super::Instrument;
use super::ToneInfo;

#[derive(Clone, Copy)]
pub enum PredefinedInstrument {
    SineGenerator,
    TriangleGenerator,
}

impl PredefinedInstrument {
    pub fn sine_generator(info: ToneInfo<TET12ConcreteTone>) -> f32 {
        use std::f64::consts::PI;
        
        let frequency = info.tone.to_frequency() as f64;
        let x = info.time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin() as f32 * info.intensity;
    }

    pub fn triangle_generator(info: ToneInfo<TET12ConcreteTone>) -> f32 {
        use std::f64::consts::PI;

        let frequency = info.tone.to_frequency() as f64;
        let x = info.time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin().asin() as f32 * info.intensity;
    }
}

impl Instrument for PredefinedInstrument {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: ToneInfo<Self::ConcreteValue>) -> f32 {
        match self {
            Self::SineGenerator => Self::sine_generator(info),
            Self::TriangleGenerator => Self::triangle_generator(info),
        }
    }
}
