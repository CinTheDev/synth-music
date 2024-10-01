use std::time::Duration;
use crate::instrument::Instrument;

// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece<T, U: Instrument> {
    pub sections: Vec<ExportSection<T, U>>,
}

pub struct ExportSection<T, U: Instrument> {
    pub tracks: Vec<ExportTrack<T, U>>,
}

// Contains raw tones
pub struct ExportTrack<T, U: Instrument> {
    pub tones: Vec<Tone<T>>,
    pub instrument: U,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone<T> {
    pub concrete_values: Vec<T>,
    pub play_duration: Duration,
    pub tone_duration: Duration,

    pub intensity: f32,
}

impl<T, U: Instrument> ExportMusicPiece<T, U> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
}

impl<T, U: Instrument> ExportSection<T, U> {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
}

impl<T, U: Instrument> ExportTrack<T, U> {
    pub fn new(instrument: U) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}
