pub mod predefined;

use std::time::Duration;

use crate::file_export::export_info::Tone;

pub struct InstrumentBuffer {
    samples: Vec<f32>,
}

pub struct BufferInfo {
    sample_rate: u32,
    tone_duration: Duration,
}

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer;
}
