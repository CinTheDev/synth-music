use super::note::{Note, ScaledValue, Length};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};

pub struct MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
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

    fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U> {
        let mut tones = Vec::new();

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
            measures: Vec::new(),
            time_signature,
            instrument,
            current_intensity: 1.0,
        }
    }

    pub fn measure(&mut self) -> Result<&mut Measure<T>, ()> {
        self.measures.push(Measure {
            time_signature: self.time_signature,
            notes: Vec::new(),
        });

        let active_measure = self.get_active_measure();

        if let Ok(_) = active_measure.assert_measure_bounds() {
            return Ok(active_measure);
        }
        else {
            return Err(());
        }
    }

    fn get_active_measure(&mut self) -> &mut Measure<T> {
        let last_index = self.measures.len() - 1;
        return &mut self.measures[last_index];
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
    fn assert_measure_bounds(&self) -> Result<(), ()> {
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

        if current_measure_length <= max_measure_length {
            return Ok(());
        }
        else {
            return Err(());
        }

        // TODO: Have panic message show line number in user space
        //assert!(current_measure_length <= max_measure_length, "Measure overflow");
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
