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
    active_samples: usize,
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
    pub fn new(samples: Vec<f32>, sample_rate: u32, active_samples: usize) -> Self {
        Self {
            samples,
            sample_rate,
            active_samples,
        }
    }

    pub fn time_from_index(&self, index: usize) -> Duration {
        Duration::from_secs_f64(
            index as f64 / self.sample_rate as f64
        )
    }

    pub fn mix(self, other: Self) -> Self {
        assert_eq!(self.sample_rate, other.sample_rate);

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
            sample_rate: self.sample_rate,
            active_samples,
        }
    }

    pub fn append(&mut self, other: Self) {
        todo!();
    }

    pub fn extend_to_active_samples(&mut self) {
        if self.active_samples < self.samples.len() { return }

        let remaining_samples = self.active_samples - self.samples.len();
        
        for _ in 0..remaining_samples {
            self.samples.push(0.0);
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn active_samples(&self) -> usize {
        self.active_samples
    }
}
