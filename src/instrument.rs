pub mod predefined;

use crate::file_export::Tone;
use std::time::Duration;

/// A simple buffer that is returned by `render_buffer` of `Instrument`.
#[derive(Clone)]
pub struct InstrumentBuffer {
    pub samples: Vec<f32>,
}

/// Information that is necessary / useful for creating a buffer in the
/// `render_buffer` function.
#[derive(Clone)]
pub struct BufferInfo {
    pub sample_rate: u32,
    pub tone_samples: usize,
}

/// Implementors can be used as instruments for Tracks.
/// 
/// This implementation takes care of the whole sound synthesis when rendering
/// a music piece. `ConcreteValue` is the type of the note system that the
/// instrument works with. The most common type is the 12-TET note system, that
/// is defined inside `tet12`.
/// 
/// The function `render_buffer` is responsible for completely rendering the
/// given tone and information. `buffer_info` provides useful info for the
/// buffer, such as the sample rate, and how many samples are expected to be
/// returned.
/// 
/// `tones` is the tone / note that needs to be generated, and contains all the
/// info of the required tone.
pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer;
}

pub trait TimeInstrument {
    type A: Clone + Copy;

    fn render_sample(&self, tones: &Tone<Self::A>) -> f32;
}


// TODO: Remove doubled implementation
impl BufferInfo {
    /// Calculate the time for an index at the `sample_rate`. If the sample rate
    /// is 44.1 kHz, then the 44100th sample is at the time of 1 second.
    pub fn time_from_index(&self, index: usize) -> Duration {
        Duration::from_secs_f64(
            index as f64 / self.sample_rate as f64
        )
    }
}
