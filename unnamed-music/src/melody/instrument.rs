pub mod note;

use note::Note;

pub struct Instrument {
    // TODO: Sound generation
    pub tracks: Vec<Track>,
}

pub struct Track {
    notes: Vec<Note>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
        }
    }

    pub fn note(&mut self, note: Note) {
        self.notes.push(note);
    }
}
