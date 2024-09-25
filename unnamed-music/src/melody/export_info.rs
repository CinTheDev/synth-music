use std::time::Duration;


// Holds all info of a piece relevant for exporting; consists of several
// export tracks
pub struct ExportMusicPiece {

}

// Contains raw tones
pub struct ExportTrack {

}

// Represents a raw tone - just a frequency, duration, and intensity
pub struct Tone {
    frequencies: Vec<f32>,
    play_duration: Duration,
    tone_duration: Duration,
    intensity: f32,
}
