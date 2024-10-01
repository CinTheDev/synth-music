use std::time::Duration;
use super::instrument::Instrument;

// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece<T, U: Instrument<U>> {
    pub sections: Vec<ExportSection<T, U>>,
}

pub struct ExportSection<T, U: Instrument<U>> {
    pub tracks: Vec<ExportTrack<T, U>>,
}

// Contains raw tones
pub struct ExportTrack<T, U: Instrument<U>> {
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

//#[derive(Clone, Copy)]
//pub struct TET12ConcreteValue(pub i32);

impl<T, U: Instrument<U>> ExportMusicPiece<T, U> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
}

impl<T, U: Instrument<U>> ExportSection<T, U> {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
}

impl<T, U: Instrument<U>> ExportTrack<T, U> {
    pub fn new(instrument: U) -> Self {
        Self {
            tones: Vec::new(),
            instrument,
        }
    }
}

/*
impl TET12ConcreteValue {
    pub fn to_frequency(self) -> f32 {
        Self::frequency_from_a4_distance(self.0)
    }

    fn frequency_from_a4_distance(semitones: i32) -> f32 {
        2_f32.powf(semitones as f32 / 12.0) * 440.0
    }
}
*/
