use std::time::Duration;
use super::instrument::Instrument;

// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece<T: Instrument> {
    pub sections: Vec<ExportSection<T>>,
}

pub struct ExportSection<T: Instrument> {
    pub tracks: Vec<ExportTrack<T>>,
}

// Contains raw tones
pub struct ExportTrack<T: Instrument> {
    pub tones: Vec<Tone>,
    pub instrument: T,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone {
    pub frequencies: Vec<f32>,
    pub play_duration: Duration,
    pub tone_duration: Duration,
    pub intensity: f32,
}

impl<T: Instrument> ExportMusicPiece<T> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
}

impl<T: Instrument> ExportSection<T> {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
}

impl<T: Instrument> ExportTrack<T> {
    pub fn new(instrument: T) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}
