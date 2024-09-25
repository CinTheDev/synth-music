use std::time::Duration;


// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece {
    tracks: Vec<ExportTrack>,
}

// Contains raw tones
pub struct ExportTrack {
    // TODO: Sound generator
    tones: Vec<Tone>,
}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone {
    frequencies: Vec<f32>,
    play_duration: Duration,
    tone_duration: Duration,
    intensity: f32,
}
