pub mod note;
pub mod music_key;

use note::{Note, Length, ScaledValue};
use crate::instrument::Instrument;

#[derive(Clone)]
pub struct Track<T: ScaledValue, U: Instrument> {
    notes: Vec<Note<T>>,
    instrument: U,

    current_intensity: f32,
}

impl<T: ScaledValue, U: Instrument> Track<T, U> {
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

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
/*
pub struct Composition<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub sections: Vec<Section<T, U>>,
}

#[derive(Clone, Copy)]
pub struct SectionInfo {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),
}

#[derive(Clone)]
pub struct Section<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub info: SectionInfo,
    pub tracks: Vec<Track<T, U>>,
}

impl<T, U> Composition<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub fn to_export_piece(self) -> ExportMusicPiece<U> {
        let mut result = ExportMusicPiece::new();

        for section in self.sections {
            let export_section = Self::generate_export_section(section);
            result.sections.push(export_section);
        }

        return result;
    }

    fn generate_export_section(section: Section<T, U>) -> ExportSection<U> {
        let mut export_section = ExportSection::new();

        for track in section.tracks {
            let export_track = Self::generate_export_track(track, section.info);
            export_section.tracks.push(export_track);
        }

        return export_section;
    }

    fn generate_export_track(track: Track<T, U>, section_info: SectionInfo) -> ExportTrack<U> {
        let (notes, instrument) = track.into_parts();

        let mut export_track = ExportTrack::new(instrument);

        for note in notes {
            let tone = Self::generate_tone(note, section_info);
            export_track.tones.push(tone);
        }

        return export_track;
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
*/
