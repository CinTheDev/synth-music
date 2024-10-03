use super::note::{Note, ScaledValue, Length};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};

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
