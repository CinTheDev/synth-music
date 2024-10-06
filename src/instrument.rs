pub mod predefined;

use crate::file_export::export_info::Tone;
use std::time::Duration;

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


// TODO: Remove doubled implementation
impl BufferInfo {
    pub fn time_from_index(&self, index: usize) -> Duration {
        Duration::from_secs_f64(
            index as f64 / self.sample_rate as f64
        )
    }
}
