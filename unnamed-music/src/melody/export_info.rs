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
    pub concrete_values: Vec<ConcreteValue>,
    pub play_duration: Duration,
    pub tone_duration: Duration,

    pub intensity: f32,
    //pub fade_in: Duration,
    //pub fade_out: Duration,
}

#[derive(Clone, Copy)]
pub struct ConcreteValue(pub i32);

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

impl ConcreteValue {
    pub fn to_frequency(self) -> f32 {
        Self::frequency_from_a4_distance(self.0)
    }

    fn frequency_from_a4_distance(semitones: i32) -> f32 {
        2_f32.powf(semitones as f32 / 12.0) * 440.0
    }
}
