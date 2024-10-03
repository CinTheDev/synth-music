use super::note::{Note, ScaledValue, Length};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};

pub struct MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    active_measure: Option<Measure<T>>,
    measures: Vec<Measure<T>>,
    time_signature: (u8, u8),
    instrument: U,

    current_intensity: f32,
}

pub struct Measure<T: ScaledValue> {
    time_signature: (u8, u8),
    notes: Vec<Note<T>>,
}

impl<T, U> MusicTrack<T, U> for MeasureTrack<T, U>
where
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T> {
        let intensity = self.current_intensity;
        let active_measure = self.get_active_measure();
        active_measure.notes.push(Note {
            values,
            length,
            play_fraction: 1.0,
            intensity,
            dotted: false,
            triole: false,
        });

        let last_index = active_measure.notes.len() - 1;
        return &mut active_measure.notes[last_index];
    }

    fn note(&mut self, length: Length, value: T) -> &mut Note<T> {
        self.notes(length, vec![value])
    }

    fn pause(&mut self, length: Length) -> &mut Note<T> {
        self.notes(length, vec![])
    }

    fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }

    fn convert_to_export_track(mut self, section_info: SectionInfo) -> ExportTrack<U> {
        let mut tones = Vec::new();

        if ! &self.get_active_measure().is_empty() {
            eprintln!("WARNING: Unvalidated measure, you probably forgot to call track.measure() at the end.");
        }

        for measure in self.measures {
            for note in measure.notes {
                let tone = Self::generate_tone(note, section_info);
                tones.push(tone);
            }
        }

        ExportTrack {
            tones,
            instrument: self.instrument,
        }
    }
}

impl<T, U> MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    pub fn new(instrument: U, time_signature: (u8, u8)) -> Self {
        Self {
            active_measure: Some(Measure::new(time_signature)),
            measures: Vec::new(),
            time_signature,
            instrument,
            current_intensity: 1.0,
        }
    }

    pub fn measure(&mut self) -> Result<&mut Measure<T>, ()> {
        let active_measure_valid = self.get_active_measure().assert_measure_bounds();

        if !active_measure_valid {
            return Err(());
        }

        let new_measure = Measure::new(self.time_signature);
        let valid_measure = self.active_measure.replace(new_measure).unwrap();

        self.measures.push(valid_measure);
        let last_index = self.measures.len() - 1;
        return Ok(&mut self.measures[last_index]);
    }

    fn get_active_measure(&mut self) -> &mut Measure<T> {
        self.active_measure.as_mut().unwrap()
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

impl<T: ScaledValue> Measure<T> {
    fn new(time_signature: (u8, u8)) -> Self {
        Self {
            time_signature,
            notes: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    fn assert_measure_bounds(&self) -> bool {
        let max_measure_length = match self.time_signature.1 {
            1 => 16,
            2 => 8,
            4 => 4,
            8 => 2,
            16 => 1,

            _ => panic!("Invalid or unsupported time signature"),
        } * self.time_signature.0 as u32;

        let mut current_measure_length = 0;

        for note in &self.notes {
            current_measure_length += Self::note_length_smallest(note);
        }

        return current_measure_length <= max_measure_length;
    }

    fn note_length_smallest(note: &Note<T>) -> u32 {
        // Smallest length right now is sixteenth

        let mut length = match note.length {
            Length::Whole => 16,
            Length::Half => 8,
            Length::Quarter => 4,
            Length::Eigth => 2,
            Length::Sixteenth => 1,
        };

        if note.dotted {
            length = length * 2 - length / 2;
        }

        // TODO: Triole

        return length;
    }

    pub fn override_time_signature(&mut self, time_signature: (u8, u8)) -> &mut Self {
        self.time_signature = time_signature;
        self
    }
}
