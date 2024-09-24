pub mod note;

use note::Note;

pub struct Instrument {
    // TODO: Sound generation
    pub tracks: Vec<Track>,
}

pub struct Track {
    notes: Vec<Note>,
}
