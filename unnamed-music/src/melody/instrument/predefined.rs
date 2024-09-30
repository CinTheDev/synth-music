use super::Instrument;
use super::ToneInfo;

pub struct SineGenerator;

pub struct TriangleGenerator;

pub enum PredefinedInstrument {
    SineGenerator(SineGenerator),
    TriangleGenerator(TriangleGenerator),
}

impl PredefinedInstrument {
    pub fn sine_generator(attributes: &SineGenerator, info: ToneInfo) -> f32 {
        todo!();
    }

    pub fn triangle_generator(attributes: &TriangleGenerator, info: ToneInfo) -> f32 {
        todo!();
    }
}

impl Instrument for PredefinedInstrument {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        match self {
            Self::SineGenerator(attributes) => Self::sine_generator(attributes, info),
            Self::TriangleGenerator(attributes) => Self::triangle_generator(attributes, info),
        }
    }
}
