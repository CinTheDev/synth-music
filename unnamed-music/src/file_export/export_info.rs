use std::time::Duration;
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

    pub intensity: f32,
}

impl<T: Instrument> ExportTrack<T> {
    pub fn new(instrument: T) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}
