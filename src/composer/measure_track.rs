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
    instrument: U,

    current_intensity: f32,
}

pub struct Measure<T: ScaledValue> {
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
        todo!();
    }
}

impl<T, U> MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    pub fn new(instrument: U) -> Self {
        Self {
            measures: Vec::new(),
            instrument,
            current_intensity: 1.0,
        }
    }

    pub fn measure(&mut self) -> &mut Measure<T> {
        self.measures.push(Measure {
            notes: Vec::new(),
        });

        self.get_active_measure()
    }

    fn get_active_measure(&mut self) -> &mut Measure<T> {
        let last_index = self.measures.len() - 1;
        return &mut self.measures[last_index];
    }
}
