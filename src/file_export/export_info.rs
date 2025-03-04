use std::time::Duration;
use std::ops::Range;
use crate::instrument::Instrument;
use crate::composer::MusicKey;

/// The export version of a track. All `MusicTrack` types are able to convert to
/// this.
/// 
/// It only contains the assigned instrument and all the tones of the track.
pub struct ExportTrack<T: Instrument> {
    pub tones: Vec<Tone<T::ConcreteValue>>,
    pub instrument: T,
}

/// A raw tone. This is essentailly the export version of a `Note`.
/// 
/// The values are conrete values; again, no values are a pause, one value is a
/// single tone, and multiple values are multiple tones played at once.
/// 
/// The length is converted to a duration. The intensity is represented as a
/// range, where the start intensity is the intensity at the beginning, and the
/// end intensity is the intensity at the end of the duration.
/// 
/// There's also an additional field `beat_emphasis`. If there's a value there,
/// the note lies on a beat specified in the time signature, and contains the
/// emphasis level. If there's no value then the note is an offbeat.
pub struct Tone<T> {
    pub concrete_values: Vec<T>,
    // TODO: Extract all properties into seperate struct
    pub play_duration: Duration,
    pub tone_duration: Duration,
    pub intensity: Range<f32>,
}

/// A collection of important values that are global for the entire composition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompositionSettings {
    pub sample_rate: u32,
}

/// A collection of important values that are local to a section. Unlike
/// `CompositionSettings`, these values can change throughout the composition.
#[derive(Clone, Copy)]
pub struct SectionInfo<'a> {
    pub bpm: f32,
    pub key: MusicKey,

    pub settings: &'a CompositionSettings,
}

/// A buffer that holds samples of rendered sections.
/// 
/// There is additional info stored used for correctly mixing or appending two
/// buffers together.
#[derive(Clone)]
pub struct SoundBuffer {
    pub samples: Vec<f32>,
    active_samples: usize,
    settings: CompositionSettings,
}

impl<T: Instrument> ExportTrack<T> {
    pub fn new(instrument: T) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}

impl SoundBuffer {
    /// Create a new SoundBuffer with the specified values. This shouldn't be
    /// used by the user.
    // TODO: Use something safer and more convenient
    pub fn new(settings: CompositionSettings) -> Self {
        Self {
            samples: Vec::new(),
            active_samples: 0,
            settings,
        }
    }

    pub fn from_parts(samples: Vec<f32>, active_samples: usize, settings: CompositionSettings) -> Self {
        Self {
            samples,
            active_samples,
            settings,
        }
    }

    /// Calculate the point in time from a specific index using the sample rate
    pub fn time_from_index(&self, index: usize) -> Duration {
        Duration::from_secs_f64(
            index as f64 / self.settings.sample_rate as f64
        )
    }

    /// Add two buffers together. The length of both buffers does not need to be
    /// the same.
    pub fn mix(self, other: Self) -> Self {
        assert_eq!(self.settings, other.settings);

        let (mut larger_buffer, smaller_buffer) =
            match self.samples.len() >= other.samples.len() {
                true => (self.samples, other.samples),
                false => (other.samples, self.samples),
            };

        for i in 0..smaller_buffer.len() {
            larger_buffer[i] += smaller_buffer[i];
        }

        let active_samples = usize::max(self.active_samples, other.active_samples);

        Self {
            samples: larger_buffer,
            active_samples,
            settings: self.settings,
        }
    }

    /// Append the given buffer to the current buffer.
    /// 
    /// Depending on the internal state, this function might partially mix both
    /// buffers (e.g. when one buffer has more samples because of reverb).
    pub fn append(&mut self, other: Self) {
        let inactive_samples = self.samples.len() - self.active_samples;

        let mix_samples = usize::min(inactive_samples, other.samples.len());

        // Mix end of self and start of other
        for i in 0..mix_samples {
            let index_self = i + self.active_samples;
            let index_other = i;

            self.samples[index_self] += other.samples[index_other];
        }

        self.active_samples += other.active_samples;

        // If other has been fully mixed in already
        if inactive_samples > other.samples.len() { return }

        let remaining_buffer = &other.samples[inactive_samples..];
        for value in remaining_buffer {
            self.samples.push(*value);
        }
    }

    /// Transition from the current buffer to the given buffer.
    /// 
    /// This acts similar as `append()`, but instead of adding mixed samples,
    /// they are averaged.
    pub fn transition(&mut self, other: Self) {
        let inactive_samples = self.samples.len() - self.active_samples;

        let mix_samples = usize::min(inactive_samples, other.samples.len());

        // Mix end of self and start of other
        for i in 0..mix_samples {
            let index_self = i + self.active_samples;
            let index_other = i;

            //self.samples[index_self] += other.samples[index_other];
            let average = (self.samples[index_self] + other.samples[index_other]) / 2.0;
            self.samples[index_self] = average;
        }

        self.active_samples += other.active_samples;

        // If other has been fully mixed in already
        if inactive_samples > other.samples.len() { return }

        let remaining_buffer = &other.samples[inactive_samples..];
        for value in remaining_buffer {
            self.samples.push(*value);
        }
    }

    /// If the buffer is shorter than expected, extend the buffer with silence
    /// until the expected length is met.
    pub fn extend_to_active_samples(&mut self) {
        if self.active_samples < self.samples.len() { return }

        let remaining_samples = self.active_samples - self.samples.len();
        
        for _ in 0..remaining_samples {
            self.samples.push(0.0);
        }
    }

    /// Retrieve the internally saved settings
    pub fn settings(&self) -> CompositionSettings {
        self.settings
    }

    /// Retrieve the amount of "active samples".
    /// 
    /// These "active samples" describe how long this buffer is expected to be
    /// / how much time it takes up in the composition. This value is used
    /// internally, and the user shouldn't need to work with this.
    pub fn active_samples(&self) -> usize {
        self.active_samples
    }

    /// Set the amount of "active samples".
    /// 
    /// Only overwrite if you know what you're doing, as wrong values will break
    /// synchronization of the tones.
    pub fn set_active_samples(&mut self, value: usize) {
        self.active_samples = value;
    }

    /// Will adjust the intensity of every sample so that the loudest sample
    /// will be at 1.0 (or -1.0). This effectively removes clipping artifacts
    /// at the cost of making the intensity scale relative.
    /// 
    /// Equivalent to `normalize`; this variant consumes and returns self.
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    /// Will adjust the intensity of every sample so that the loudest sample
    /// will be at 1.0 (or -1.0). This effectively removes clipping artifacts
    /// at the cost of making the intensity scale relative.
    /// 
    /// Equivalent to `normalized`; this variant operates on a mutable
    /// reference.
    pub fn normalize(&mut self) {
        let loudest_sample = Self::find_loudest_sample(&self);

        for sample in self.samples.iter_mut() {
            *sample /= loudest_sample;
        }
    }

    fn find_loudest_sample(&self) -> f32 {
        let mut loudest = 0.0;

        for sample in &self.samples {
            let amplitude = sample.abs();
            if amplitude > loudest {
                loudest = amplitude;
            }
        }

        return loudest;
    }
}

mod tests;
