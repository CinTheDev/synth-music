pub mod note;
use note::Note;
use note::scaled_value::ScaledValue;
use crate::melody::instrument::Instrument;

#[derive(Clone)]
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

    pub fn into_parts(self) -> (Vec<Note>, T) {
        (self.notes, self.instrument)
    }

    pub fn note(
        &mut self,
        length: note::Length,
        value: ScaledValue,
    ) -> &mut Note {
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
        values: Vec<ScaledValue>,
    ) -> &mut Note {
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
