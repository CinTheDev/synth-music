pub mod note;
pub mod music_key;
pub mod unbound_track;
pub mod measure_track;
pub mod time_signature;

use crate::instrument::Instrument;

#[doc(inline)]
pub use note::{Note, length, Length, ScaledValue};
#[doc(inline)]
pub use time_signature::TimeSignature;
#[doc(inline)]
pub use music_key::{MusicKey, KeyTonic, KeyType};
#[doc(inline)]
pub use unbound_track::UnboundTrack;
#[doc(inline)]
pub use measure_track::MeasureTrack;
#[doc(inline)]
pub use crate::file_export::export_info::{ExportTrack, SectionInfo, Tone};

/// A trait for the general behaviour of Tracks that expose functionality for
/// placing notes. Only implement this as an alternative to MeasureTrack or
/// UnboundTrack, if these provided implementations don't satisfy your needs.
/// 
/// # Implementation details
/// 
/// ## pause, note, notes
/// 
/// These functions all place `Note` structs in the track. A `Note` represents
/// either a pause, a single note, or multiple stacked notes. All functions take
/// a length for the note to occupy. These functions can be called by the user,
/// and are called by the Track macros like `sequential_notes!` or `notes!`.
/// They also have to return a mutable reference to the placed note.
/// 
/// ## start_dynamic_change, end_dynamic_change
/// 
/// Mark the start and end of a dynamics change. The dynamics must interpolate
/// over the marked region, going from the currently active intensity to the
/// intensity specified in end_dynamic_change.
/// 
/// ## set_intensity, set_play_fraction
/// 
/// Set the currently active property (intensity, play_fraction). Only notes
/// placed after the function call are affected; all notes that are already
/// placed remain unchanged.
/// 
/// ## get_active_note
/// 
/// Retrieves a mutable reference to the most recent note placed on the track.
/// The return value is wrapped in an Option in case there are no notes placed.
/// 
/// 
/// ## convert_to_export_track
/// 
/// Usually called by the render macros / functions. An export track is an
/// alternative representation of the track that is convenient for rendering.
/// 
/// Please refer to the `ExportTrack` documentation for details on the
/// conversion and structure of it.
/// 
pub trait MusicTrack<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    fn pause(&mut self, length: Length) -> &mut Note<T>;
    fn note(&mut self, length: Length, value: T) -> &mut Note<T>;
    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T>;

    fn start_dynamic_change(&mut self);
    fn end_dynamic_change(&mut self, intensity: f32);

    fn set_intensity(&mut self, intensity: f32);
    fn set_play_fraction(&mut self, play_fraction: f32);

    fn get_active_note(&mut self) -> Option<&mut Note<T>>;

    fn convert_to_export_track(&self, section_info: SectionInfo) -> ExportTrack<U>;
}

/// Place stacked notes on a track.
#[macro_export]
macro_rules! notes {
    ( $track:expr, $len:expr, $( $args:expr ),* $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($args);
            )*
            $track.notes($len, temp_vec)
        }
    };
}

/// Place a lot of notes sequentially on a track. All notes are individual notes
/// (no stacked notes), and the length will be the same for every note.
/// 
/// Please be careful with this when using a `MeasureTrack`, the placed notes
/// cannot go beyond measure boundaries.
#[macro_export]
macro_rules! sequential_notes {
    ( $track:expr, $len:expr, $( $args:expr ),+ $(,)? ) => {
        $(
            $track.note($len, $args);
        )*
    };
}
