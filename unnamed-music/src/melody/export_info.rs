use std::time::Duration;

// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece {
    pub tracks: Vec<ExportTrack>,
}

// Contains raw tones
pub struct ExportTrack {
    // TODO: Sound generator
    pub tones: Vec<Tone>,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone {
    pub frequencies: Vec<f32>,
    pub play_duration: Duration,
    pub tone_duration: Duration,
    pub intensity: f32,
}

impl ExportMusicPiece {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
}

impl ExportTrack {
    pub fn new() -> Self {
        Self {
            tones: Vec::new(),
        }
    }
}
