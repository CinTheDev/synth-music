use crate::melody::composer::music_key::{MusicKey, MusicKeyBase, MusicKeyType};

#[derive(Clone, Copy)]
pub struct ScaledValue {
    index: u8,
    octave: i32,
    offset: i32,
}

impl ScaledValue {
    pub fn sharp(mut self) -> Self {
        self.offset += 1;
        self
    }

    pub fn flat(mut self) -> Self {
        self.offset -= 1;
        self
    }

    pub fn get_concrete_value(self, key: MusicKey) -> i32 {
        let distance_from_tonic = Self::get_distance_from_tonic(key.key_type, self.index);
        let distance_tonic_from_a4 = Self::distance_from_a4(key.base);
        let octave_offset = self.octave * 12;
        
        return 
            distance_from_tonic
            + distance_tonic_from_a4
            + octave_offset
            + self.offset;
    }

    fn distance_from_a4(tonic: MusicKeyBase) -> i32 {
        match tonic {
            // TODO: D#
            MusicKeyBase::D      =>  5,
            MusicKeyBase::Dflat  =>  4,
            MusicKeyBase::Csharp =>  4,
            MusicKeyBase::C      =>  3,
            MusicKeyBase::B      =>  2,
            MusicKeyBase::Bflat  =>  1,
            // TODO: A#
            MusicKeyBase::A      =>  0,
            MusicKeyBase::Aflat  => -1,
            // TODO: G#
            MusicKeyBase::G      => -2,
            MusicKeyBase::Gflat  => -3,
            MusicKeyBase::Fsharp => -3,
            MusicKeyBase::F      => -4,
            MusicKeyBase::E      => -5,
            MusicKeyBase::Eflat  => -6,
        }
    }

    fn get_distance_from_tonic(key_type: MusicKeyType, index: u8) -> i32 {
        match key_type {
            MusicKeyType::Major => Self::get_distance_major(index),
            MusicKeyType::Minor => Self::get_distance_minor(index),
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

pub fn first(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 0,
        octave,
        offset: 0,
    }
}

pub fn second(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 1,
        octave,
        offset: 0,
    }
}

pub fn third(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 2,
        octave,
        offset: 0,
    }
}

pub fn fourth(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 3,
        octave,
        offset: 0,
    }
}

pub fn fifth(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 4,
        octave,
        offset: 0,
    }
}

pub fn sixth(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 5,
        octave,
        offset: 0,
    }
}

pub fn seventh(octave: i32) -> ScaledValue {
    ScaledValue {
        index: 6,
        octave,
        offset: 0,
    }
}
