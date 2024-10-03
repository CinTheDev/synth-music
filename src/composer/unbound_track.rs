use super::note::{Note, ScaledValue, Length};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};

#[derive(Clone)]
pub struct UnboundTrack<T: ScaledValue, U: Instrument> {
    notes: Vec<Note<T>>,
    instrument: U,

    current_intensity: f32,
}

impl<T, U> MusicTrack<T, U> for UnboundTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    fn pause(&mut self, length: Length) -> &mut Note<T> {
        self.notes(length, vec![])
    }

    fn note(&mut self, length: Length, value: T) -> &mut Note<T> {
        self.notes(length, vec![value])
    }

    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T> {
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

    fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }

    fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U> {
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
}

impl<T, U> UnboundTrack<T, U>
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
