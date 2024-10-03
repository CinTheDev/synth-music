pub mod note;
pub mod music_key;

use note::{Note, Length, ScaledValue};
use music_key::MusicKey;
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};

#[derive(Clone, Copy)]
pub struct SectionInfo {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),
}

pub trait MusicTrack<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    fn pause(&mut self, length: Length) -> &mut Note<T>;
    fn note(&mut self, length: Length, value: T) -> &mut Note<T>;
    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T>;

    fn set_intensity(&mut self, intensity: f32);

    fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U>;
}

#[derive(Clone)]
pub struct Track<T: ScaledValue, U: Instrument> {
    notes: Vec<Note<T>>,
    instrument: U,

    current_intensity: f32,
}

impl<T, U> Track<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    pub fn new(instrument: U) -> Self {
        Self {
            notes: Vec::new(),
            instrument,
            current_intensity: 1.0,
        }
    }

    pub fn note(
        &mut self,
        length: Length,
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
        length: Length,
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

    pub fn pause(&mut self, length: Length) {
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

    pub fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U> {
        let mut tones = Vec::new();

        for note in self.notes {
            let tone = Self::generate_tone(note, section_info);
            tones.push(tone);
        }

        ExportTrack {
            tones,
            instrument: self.instrument,
        }
    }

    fn generate_tone(note: Note<T>, section_info: SectionInfo) -> Tone<U::ConcreteValue> {
        let mut concrete_values = Vec::new();

        for scaled_value in &note.values {
            let concrete_value = scaled_value.to_concrete_value(section_info.key);
            concrete_values.push(concrete_value);
        }

        let play_duration = note.get_duration(section_info.bpm);
        let tone_duration = play_duration.mul_f32(note.play_fraction);

        Tone {
            concrete_values,
            play_duration,
            tone_duration,
            intensity: note.intensity,
        }
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
