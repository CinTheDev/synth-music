pub mod predefined;

use crate::file_export::Tone;
use crate::file_export::SoundBuffer;
use std::time::Duration;

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

    fn render(&self, tones: &Tone<Self::ConcreteValue>, buffer: &mut SoundBuffer) {
        let mut tone_buffers = Vec::with_capacity(tones.concrete_values.len() + 1);
        let num_samples = self.get_num_samples(&buffer, tones);

        let empty = SoundBuffer::from_parts(
            vec![0.0; num_samples],
            buffer.active_samples(),
            buffer.settings()
        );
        tone_buffers.push(empty);

        for tone in &tones.concrete_values {
            let mut tone_buffer = SoundBuffer::from_parts(
                Vec::with_capacity(num_samples),
                buffer.active_samples(),
                buffer.settings()
            );
            self.render_tone_buffer(*tone, &mut tone_buffer, num_samples);
            tone_buffers.push(tone_buffer);
        }

        self.mix_tone_samples(tone_buffers, buffer);

        self.apply_intensity(tones, buffer);
        self.post_process(tones, buffer);
    }

    fn render_tone_buffer(
        &self,
        tone: Self::ConcreteValue,
        buffer: &mut SoundBuffer,
        num_samples: usize,
    ) {
        for i in 0..num_samples {
            let time = buffer.time_from_index(i);

            let sample = self.render_sample(tone, time);
            buffer.samples.push(sample);
        }
    }

    fn render_sample(&self, _tone: Self::ConcreteValue, _time: Duration) -> f32 { 0.0 }

    fn get_num_samples(&self, buffer_info: &SoundBuffer, _tones: &Tone<Self::ConcreteValue>) -> usize {
        buffer_info.active_samples()
    }

    fn mix_tone_samples(&self, tone_buffers: Vec<SoundBuffer>, out_buffer: &mut SoundBuffer) {
        for tone_buffer in tone_buffers {
            *out_buffer = tone_buffer.mix(out_buffer.clone());
        }
    }

    fn apply_intensity(&self, tones: &Tone<Self::ConcreteValue>, buffer: &mut SoundBuffer) {
        for i in 0..buffer.samples.len() {
            let time = buffer.time_from_index(i);
            let intensity = self.get_intensity(tones, time);
            buffer.samples[i] *= intensity;
        }
    }

    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let t = time.as_secs_f32() / tones.play_duration.as_secs_f32();
        let intensity = &tones.intensity;

        return t * (intensity.end - intensity.start) + intensity.start;
    }

    fn post_process(&self, _tones: &Tone<Self::ConcreteValue>, _buffer: &mut SoundBuffer) { }
}
