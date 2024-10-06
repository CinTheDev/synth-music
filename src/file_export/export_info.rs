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
    pub fn new(sample_rate: u32) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
        }
    }

    pub fn get_time_from_index(&self, index: u32) -> Duration {
        let secs = index as f64 / self.sample_rate as f64;
        Duration::from_secs_f64(secs)
    }

    pub fn append(&mut self, other: &mut SoundBuffer) {
        self.samples.append(&mut other.samples);
    }
}
