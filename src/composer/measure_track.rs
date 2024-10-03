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
    measures: Vec<Measue<T>>,
    instrument: U,

    current_intensity: f32,
}

pub struct Measue<T: ScaledValue> {
    notes: Vec<Note<T>>,
}
