use std::time::Duration;
use std::ops::Range;
use crate::instrument::Instrument;

// Contains raw tones
pub struct ExportTrack<T: Instrument> {
    pub tones: Vec<Tone<T::ConcreteValue>>,
    pub instrument: T,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone<T> {
    pub concrete_values: Vec<T>,
    pub play_duration: Duration,
    pub tone_duration: Duration,

    pub intensity: Range<f32>,
}

pub struct SoundBuffer {
    pub samples: Vec<f32>,
    sample_rate: u32,
    active_samples: u32,
}

impl<T: Instrument> ExportTrack<T> {
    pub fn new(instrument: T) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}

/*
impl SoundBuffer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
        }
    }

    pub fn get_time_from_index(&self, index: usize) -> Duration {
        let secs = index as f64 / self.sample_rate as f64;
        Duration::from_secs_f64(secs)
    }

    pub fn append(&mut self, other: &mut SoundBuffer) {
        self.samples.append(&mut other.samples);
    }

    pub fn preallocate(&mut self, sample_count: usize) {
        self.samples = Vec::with_capacity(sample_count);
        self.extend(sample_count);
    }

    pub fn extend(&mut self, sample_count: usize) {
        for _ in 0..sample_count {
            self.samples.push(0.0);
        }
    }

    pub fn mix(self, other: Self) -> Self {
        assert_eq!(self.sample_rate, other.sample_rate);

        let (mut larger_buffer, smaller_buffer) = match self.samples.len() >= other.samples.len() {
            true => (self.samples, other.samples),
            false => (other.samples, self.samples),
        };

        for i in 0..smaller_buffer.len() {
            larger_buffer[i] += smaller_buffer[i];
        }

        return Self {
            samples: larger_buffer,
            sample_rate: self.sample_rate,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
*/
