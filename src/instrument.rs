pub mod predefined;

use crate::file_export::export_info::Tone;

pub struct InstrumentBuffer {
    pub samples: Vec<f32>,
}

pub struct BufferInfo {
    pub sample_rate: u32,
    pub tone_samples: usize,
}

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer;
}
