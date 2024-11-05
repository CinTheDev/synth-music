use super::MusicKey;

pub mod length;
#[doc(inline)]
pub use length::Length;

/// Represents an abstract note value that is dependent on the music key. Take
/// a look at `tet12` for an example of how this might look like.
/// 
/// The `ScaledValue` needs implement a function to convert it into a
/// `ConcreteValue` given a specific `MusicKey`. This is called during the
/// export stage.
pub trait ScaledValue: Clone {
    type ConcreteValue;

    fn to_concrete_value(&self, key: MusicKey) -> Self::ConcreteValue;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DynamicsFlag {
    None,
    StartChange,
    EndChange,
}

/// An abstract note representation
/// 
/// This is used internally to represent notes placed on a track. A note can
/// have any number of associated note values. No values represent a pause, one
/// value is a single note, and multiple are stacked notes. The note values are
/// generic and can be user-defined.
/// 
/// A note has a specific `Length` that it takes up on the Track, and a
/// `play_fraction`, which is the percentage for how long the note is being
/// played inside the length duration. A large value (e.g. 0.95) is a held note
/// while a small value (e.g. 0.1) is a shortly played note (staccato).
/// 
/// The note also stores info about dynamics like the `intensity` or in which
/// state of a dynamics change it is.
/// 
/// A `Note` is converted into a `Tone` in the export stage.
#[derive(Clone)]
pub struct Note<T: ScaledValue> {
    pub values: Vec<T>,
    pub length: Length,
    pub play_fraction: f32,

    pub intensity: f32,
    pub beat_emphasis: Option<f32>,
    pub dynamics_flag: DynamicsFlag,
}

impl<T: ScaledValue> Default for Note<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            length: length::QUARTER,
            play_fraction: 1.0,
            
            intensity: 1.0,
            beat_emphasis: Some(1.0),
            dynamics_flag: DynamicsFlag::None,
        }
    }
}

impl<T: ScaledValue> Note<T> {
    /// Makes the `play_fraction` short so that the note is only played for a
    /// small fraction of it's length.
    pub fn staccato(&mut self) -> &mut Self {
        self.play_fraction = 0.2;
        self
    }

    /// Get a duration for the note length given the tempo.
    pub fn get_duration(&self, bpm: f32) -> std::time::Duration {
        let quarters_per_second = bpm / 60.0;
        let note_length = self.length.to_float();
        let time = (4.0 * note_length) / quarters_per_second;

        return std::time::Duration::from_secs_f32(time);
    }
}
