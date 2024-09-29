use std::time::Duration;
use super::instrument::Instrument;

// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece {
    pub sections: Vec<ExportSection>,
}

pub struct ExportSection {
    pub tracks: Vec<ExportTrack>,
}

// Contains raw tones
pub struct ExportTrack {
    pub tones: Vec<Tone>,
    pub instrument: Box<dyn Instrument>,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone {
    pub concrete_values: Vec<i32>,
    pub play_duration: Duration,
    pub tone_duration: Duration,

    pub intensity: f32,
    //pub fade_in: Duration,
    //pub fade_out: Duration,
}

impl ExportMusicPiece {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
}

impl ExportSection {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
}

impl ExportTrack {
    pub fn new(instrument: Box<dyn Instrument>) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}
