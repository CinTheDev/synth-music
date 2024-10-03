use super::note::{Note, ScaledValue, Length};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};


/*
 * MeasureTrack syntax notes (How should the user "see" the measures?)
 * 
 * It would be best if these measures are visible at compile time and not at
 * runtime.
 * 
 * There are multiple options to make measures visible:
 * 
 * ## Returning measure objects and operating on those
 * 
 * Probably won't be compatible with the trait stuff as the Track itself needs
 * to place those notes
 * 
 * ## Using functions to mark end and start of measures
 * 
 * Better, but how are we going to enforce measure bounds?
 * 
 * If we use stuff like panic!(), the measure bounds are only enforced at
 * runtime.
 * 
 * As there are no better alternatives using short syntax, let's try the panic!()
 * stuff.
 */
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

        active_measure.assert_measure_bounds();

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

    pub fn measure(&mut self) -> &mut Measure<T> {
        self.measures.push(Measure {
            time_signature: self.time_signature,
            notes: Vec::new(),
        });

        self.get_active_measure()
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
    fn assert_measure_bounds(&self) {
        todo!();
    }
}
