use super::Instrument;
use super::ToneInfo;

pub enum PredefinedInstrument {
    SineGenerator,
    TriangleGenerator,
}

impl PredefinedInstrument {
    pub fn sine_generator(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        
        let frequency = info.tone.to_frequency() as f64;
        let x = info.time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin() as f32 * info.intensity;
    }

    pub fn triangle_generator(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;

        let frequency = info.tone.to_frequency() as f64;
        let x = info.time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin().asin() as f32 * info.intensity;
    }
}

impl Instrument for PredefinedInstrument {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        match self {
            Self::SineGenerator => Self::sine_generator(info),
            Self::TriangleGenerator => Self::triangle_generator(info),
        }
    }
}
