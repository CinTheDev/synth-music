pub mod note;

use note::{Note, ScaledValue};
//use note::scaled_value::TET12ScaledValue;
use crate::melody::instrument::Instrument;

#[derive(Clone)]
pub struct Track<T: ScaledValue<T>, U: Instrument<U>> {
    notes: Vec<Note<T>>,
    instrument: U,

    current_intensity: f32,
}

impl<T: ScaledValue<T>, U: Instrument<U>> Track<T, U> {
    pub fn new(instrument: U) -> Self {
        Self {
            notes: Vec::new(),
            instrument,
            current_intensity: 1.0,
        }
    }

    pub fn into_parts(self) -> (Vec<Note<T>>, U) {
        (self.notes, self.instrument)
    }

    pub fn note(
        &mut self,
        length: note::Length,
        value: T,
    ) -> &mut Note<T> {
        self.notes.push(Note {
            values: vec![value],
            length,
            play_fraction: 1.0,
            intensity: self.current_intensity,
            dotted: false,
            triole: false,
        });

        let last_index = self.notes.len() - 1;
        return &mut self.notes[last_index];
    }

    pub fn notes(
        &mut self,
        length: note::Length,
        values: Vec<T>,
    ) -> &mut Note<T> {
        self.notes.push(Note {
            values,
            length,
            play_fraction: 1.0,
            intensity: self.current_intensity,
            dotted: false,
            triole: false,
        });

        let last_index = self.notes.len() - 1;
        return &mut self.notes[last_index];
    }

    pub fn pause(&mut self, length: note::Length) {
        self.notes.push(Note {
            values: vec![],
            length,
            play_fraction: 1.0,
            intensity: self.current_intensity,
            dotted: false,
            triole: false,
        });
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }
}

#[macro_export]
macro_rules! notes {
    ( $track:expr, $len:expr, $( $args:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($args);
            )*
            $track.notes($len, temp_vec)
        }
    };
}

#[macro_export]
macro_rules! sequential_notes {
    ( $track:expr, $len:expr, $( $args:expr ),+ ) => {
        $(
            $track.note($len, $args);
        )*
    };
}
