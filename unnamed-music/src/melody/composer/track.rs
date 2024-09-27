pub mod note;
use note::Note;
use crate::melody::instrument::Instrument;

pub struct Track<T: Instrument> {
    notes: Vec<Note>,
    instrument: T,

    current_intensity: f32,
}

impl<T: Instrument> Track<T> {
    pub fn new(instrument: T) -> Self {
        Self {
            notes: Vec::new(),
            instrument,
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
