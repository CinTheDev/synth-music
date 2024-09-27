pub mod note;
use note::Note;

pub struct Track {
    notes: Vec<Note>,
    // TODO: add instrument / sound generator here

    current_intensity: f32,
}

impl Track {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            current_intensity: 1.0,
        }
    }

    pub fn get_notes(self) -> Vec<Note> {
        self.notes
    }

    pub fn note(
        &mut self,
        length: note::Length,
        tone: note::Tone,
        octave: i32,
    ) -> &mut Note {
        self.notes.push(Note {
            values: vec![(tone, octave)],
            length,
            play_fraction: 1.0,
            intensity: self.current_intensity,
            semitones_offset: 0,
            dotted: false,
            triole: false,
        });

        let last_index = self.notes.len() - 1;
        return &mut self.notes[last_index];
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }
}
