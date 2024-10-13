use crate::composer::{MusicKey, KeyType, KeyTonic};
use crate::composer::ScaledValue;

/// An abstract representation of a note value dependent on a `MusicKey`.
/// Construct these with the functions `first(x)`, `second(x)`, ...
/// 
/// A `first(x)` note is the first note of the scale (or the tonic), where "x"
/// is the octave. If the scale e.g. is "C Major", the note `first(4)` is C4.
/// 
/// These can also be sharpened or flattened afterwards.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TET12ScaledTone {
    index: u8,
    octave: i32,
    offset: i32,
}

/// A concrete representation of a note value without a music key. This is
/// equivalent to e.g. keys on a piano. If a `MusicKey` is applied to a
/// `TET12ScaledTone`, this will be the result.
/// 
/// Notes in this form will be passed to instruments for generating sound.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TET12ConcreteTone(pub i32);

impl ScaledValue for TET12ScaledTone {
    type ConcreteValue = TET12ConcreteTone;

    fn to_concrete_value(&self, key: MusicKey) -> Self::ConcreteValue {
        TET12ConcreteTone(self.get_concrete_value(key))
    }
}

impl TET12ConcreteTone {
    /// Convert the tone into a frequency with unit Hz.
    pub fn to_frequency(self) -> f32 {
        Self::frequency_from_a4_distance(self.0)
    }

    fn frequency_from_a4_distance(semitones: i32) -> f32 {
        2_f32.powf(semitones as f32 / 12.0) * 440.0
    }
}

impl TET12ScaledTone {
    /// Sharpen the note value (increase the note value by one semitone). Can be
    /// called multiple times.
    pub fn sharp(mut self) -> Self {
        self.offset += 1;
        self
    }

    /// Flatten the note value (decrease the note value by one semitone). Can be
    /// called multiple times.
    pub fn flat(mut self) -> Self {
        self.offset -= 1;
        self
    }

    fn get_concrete_value(self, key: MusicKey) -> i32 {
        let distance_from_tonic = Self::get_distance_from_tonic(key.key_type, self.index);
        let distance_tonic_from_a4 = Self::distance_from_a4(key.tonic);
        let octave_offset = (self.octave - 4) * 12;

        return 
            distance_from_tonic
            + distance_tonic_from_a4
            + octave_offset
            + self.offset;
    }

    fn distance_from_a4(tonic: KeyTonic) -> i32 {
        match tonic {
            KeyTonic::B      =>  2,
            KeyTonic::Bflat  =>  1,

            KeyTonic::Asharp =>  1,
            KeyTonic::A      =>  0,
            KeyTonic::Aflat  => -1,

            KeyTonic::Gsharp => -1,
            KeyTonic::G      => -2,
            KeyTonic::Gflat  => -3,

            KeyTonic::Fsharp => -3,
            KeyTonic::F      => -4,

            KeyTonic::E      => -5,
            KeyTonic::Eflat  => -6,

            KeyTonic::Dsharp => -6,
            KeyTonic::D      => -7,
            KeyTonic::Dflat  => -8,

            KeyTonic::Csharp => -8,
            KeyTonic::C      => -9,
        }
    }

    fn get_distance_from_tonic(key_type: KeyType, index: u8) -> i32 {
        match key_type {
            KeyType::Major => Self::get_distance_major(index),
            KeyType::Minor => Self::get_distance_minor(index),
        }
    }

    fn get_distance_major(index: u8) -> i32 {
        match index {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 9,
            6 => 11,

            _ => panic!("Invalid scaled value index"),
        }
    }

    fn get_distance_minor(index: u8) -> i32 {
        match index {
            0 => 0,
            1 => 2,
            2 => 3,
            3 => 5,
            4 => 7,
            5 => 8,
            6 => 10,

            _ => panic!("Invalid scaled value index"),
        }
    }
}

/// Construct the first note of the scale
pub fn first(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 0,
        octave,
        offset: 0,
    }
}

/// Construct the second note of the scale
pub fn second(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 1,
        octave,
        offset: 0,
    }
}

/// Construct the third note of the scale
pub fn third(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 2,
        octave,
        offset: 0,
    }
}

/// Construct the fourth note of the scale
pub fn fourth(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 3,
        octave,
        offset: 0,
    }
}

/// Construct the fifth note of the scale
pub fn fifth(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 4,
        octave,
        offset: 0,
    }
}

/// Construct the sixth note of the scale
pub fn sixth(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 5,
        octave,
        offset: 0,
    }
}

/// Construct the seventh note of the scale
pub fn seventh(octave: i32) -> TET12ScaledTone {
    TET12ScaledTone {
        index: 6,
        octave,
        offset: 0,
    }
}

mod tests;
