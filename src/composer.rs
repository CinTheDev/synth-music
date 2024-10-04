pub mod note;
pub mod music_key;
pub mod unbound_track;
pub mod measure_track;

use note::{Note, Length, ScaledValue};
use music_key::MusicKey;
use crate::instrument::Instrument;
use crate::file_export::export_info::ExportTrack;

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
    fn set_play_fraction(&mut self, play_fraction: f32);

    fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U>;
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
