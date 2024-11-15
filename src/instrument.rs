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

    // TODO: Rename to something better, e.g. "render"
    fn render(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let num_samples = self.get_num_samples(&buffer_info, tones);
        let mut tone_buffers = Vec::with_capacity(tones.concrete_values.len());

        for tone in &tones.concrete_values {
            let buffer = self.render_tone_buffer(&buffer_info, *tone, num_samples);
            tone_buffers.push(buffer);
        }

        let mut mixed_samples = self.mix_tone_samples(tone_buffers);

        self.apply_intensity(&buffer_info, tones, &mut mixed_samples);
        self.post_process(&buffer_info, &mut mixed_samples);

        return InstrumentBuffer { samples: mixed_samples };
    }

    fn render_tone_buffer(
        &self,
        buffer_info: &BufferInfo,
        tone: Self::ConcreteValue,
        num_samples: usize,
    ) -> Vec<f32> {
        let mut buffer = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let time = buffer_info.time_from_index(i);

            let sample = self.render_sample(tone, time);
            buffer.push(sample);
        }

        return buffer;
    }

    fn render_sample(&self, _tone: Self::ConcreteValue, _time: Duration) -> f32 { 0.0 }

    fn get_num_samples(&self, buffer_info: &BufferInfo, _tones: &Tone<Self::ConcreteValue>) -> usize {
        buffer_info.tone_samples
    }

    // TODO: Improve this implementation
    //       best to do when buffer handling has been improved
    fn mix_tone_samples(&self, tone_buffers: Vec<Vec<f32>>) -> Vec<f32> {
        let mut result = Vec::new();

        for buffer in tone_buffers {
            for i in 0..buffer.len() {
                if i >= result.len() {
                    result.push(0.0);
                }

                result[i] += buffer[i];
            }
        }

        return result;
    }

    fn apply_intensity(&self, buffer_info: &BufferInfo, tones: &Tone<Self::ConcreteValue>, buffer: &mut [f32]) {
        for i in 0..buffer.len() {
            let time = buffer_info.time_from_index(i);
            let intensity = self.get_intensity(tones, time);
            buffer[i] *= intensity;
        }
    }

    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let t = time.as_secs_f32() / tones.play_duration.as_secs_f32();
        let intensity = t * (tones.intensity.end - tones.intensity.start) + tones.intensity.start;

        let emphasis = tones.beat_emphasis.unwrap_or(1.0);

        return intensity * emphasis;
    }

    fn post_process(&self, _buffer_info: &BufferInfo, _buffer: &mut Vec<f32>) { }
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
