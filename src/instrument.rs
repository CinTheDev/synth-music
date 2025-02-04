pub mod predefined;
pub mod noise;
pub mod eq;
pub mod curve;

use crate::file_export::Tone;
use crate::file_export::SoundBuffer;
use std::time::Duration;

/// Implementors can be used as instruments for Tracks.
/// 
/// This implementation takes care of the whole sound synthesis when rendering
/// a music piece. `ConcreteValue` is the type of the note system that the
/// instrument works with. The most common type is the 12-TET note system, that
/// is defined inside `tet12`.
pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    /// The main render function that will render all tones playing at the same
    /// time into a single buffer. The default implementation should be
    /// sufficient for almost all cases. Only override this if you want to have
    /// absolute control over the rendering.
    /// 
    /// If you look for something more straightforward, look at the other
    /// functons of this trait.
    /// 
    /// If you do override this, the other functions in this trait won't be
    /// called unless you call them yourself.
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

    /// Render a single tone into a buffer. By default this calls
    /// `render_sample()` for rendering every sample.
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

    /// Render a single sample of a single tone at a specified time. This is
    /// pretty much the standard way to implement an Instrument. If you can
    /// compute samples independent of each other this is the way to go.
    /// 
    /// If you cannot compute samples independent of each other, you should look
    /// for overriding `render_tone_buffer()` instead.
    /// 
    /// The default implementation just returns 0 for everything.
    fn render_sample(&self, _tone: Self::ConcreteValue, _time: Duration) -> f32 { 0.0 }

    /// Return the amount of samples the tone should have. Override this if you
    /// want to have an other number of samples than the note length suggests.
    /// 
    /// The default implementation returns the amount of samples to fill the
    /// entire note length, taking the play fraction into account.
    fn get_num_samples(&self, buffer_info: &SoundBuffer, _tones: &Tone<Self::ConcreteValue>) -> usize {
        buffer_info.active_samples()
    }

    /// Mix all tone buffers playing at the same time into one. The default
    /// implementation should be sufficient for pretty much all cases, but is
    /// still overridable if you want more control.
    /// 
    /// The default implementation will add the samples together.
    fn mix_tone_samples(&self, tone_buffers: Vec<SoundBuffer>, out_buffer: &mut SoundBuffer) {
        for tone_buffer in tone_buffers {
            *out_buffer = tone_buffer.mix(out_buffer.clone());
        }
    }

    /// A helper function that will use `get_intensity()` for every sample
    /// and multiply it by the result.
    fn apply_intensity(&self, tones: &Tone<Self::ConcreteValue>, buffer: &mut SoundBuffer) {
        for i in 0..buffer.samples.len() {
            let time = buffer.time_from_index(i);
            let intensity = self.get_intensity(tones, time);
            buffer.samples[i] *= intensity;
        }
    }

    /// Compute the intensity for a given point in time. Override this if you
    /// want e.g. the intensity to become quieter with time.
    /// 
    /// The default implementation will have the intensity stay the same as
    /// what it was specified, and interpolate in case it is dynamically
    /// changing.
    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let t = time.as_secs_f32() / tones.play_duration.as_secs_f32();
        let intensity = &tones.intensity;

        return t * (intensity.end - intensity.start) + intensity.start;
    }

    /// Is called at the end of the render function, it's possible to make final
    /// adjustments here with the rendered buffer. The default implementation
    /// does nothing.
    fn post_process(&self, _tones: &Tone<Self::ConcreteValue>, _buffer: &mut SoundBuffer) { }
}
