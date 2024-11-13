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

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::with_capacity(buffer_info.tone_samples);

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);

            let intensity = self.get_intensity(tones, time);
            let mut point_samples = Vec::with_capacity(tones.concrete_values.len());

            for tone in &tones.concrete_values {
                let sample = self.render_sample(*tone, time);
                point_samples.push(sample);
            }

            let mixed_samples = self.mix_tone_samples(&point_samples) * intensity;

            buffer.push(mixed_samples);
        }

        let mut buffer = InstrumentBuffer { samples: buffer };

        self.post_process(buffer_info, &mut buffer);

        return buffer;
    }

    fn render_sample(&self, _tone: Self::ConcreteValue, _time: Duration) -> f32 { 0.0 }

    fn mix_tone_samples(&self, samples: &[f32]) -> f32 {
        let mut result = 0.0;

        for sample in samples {
            result += sample;
        }

        result
    }

    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let t = time.as_secs_f32() / tones.play_duration.as_secs_f32();
        let intensity = t * (tones.intensity.end - tones.intensity.start) + tones.intensity.start;

        let emphasis = tones.beat_emphasis.unwrap_or(1.0);

        return intensity * emphasis;
    }

    fn post_process(&self, _buffer_info: BufferInfo, _buffer: &mut InstrumentBuffer) { }
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
