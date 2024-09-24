pub mod note;

use note::Note;

pub struct Instrument {
    // TODO: Sound generation
    tracks: Vec<Track>,
}

pub struct Track {
    notes: Vec<Note>,
}
