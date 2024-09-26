pub mod note;
use note::Note;

pub struct Instrument {
    // TODO: Sound generation
    pub tracks: Vec<Track>,
}

pub struct Track {
    notes: Vec<Note>,

    //current_length: note::Length,
    current_intensity: f32,
}

impl Track {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            //current_length: note::Length::Quarter,
            current_intensity: 1.0,
        }
    }

    pub fn get_notes(self) -> Vec<Note> {
        self.notes
    }

    pub fn note(&mut self, length: note::Length, tone: note::Tone) -> &mut Note {
        self.notes.push(Note {
            values: vec![tone],
            length,
            play_fraction: 1.0,
            intensity: self.current_intensity,
        });

        let last_index = self.notes.len() - 1;
        return &mut self.notes[last_index];
    }

    //pub fn set_length(&mut self, length: note::Length) {
    //    self.current_length = length;
    //}

    pub fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }
}
