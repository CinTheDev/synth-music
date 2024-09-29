use crate::melody::composer::{MusicKey, MusicKeyBase, MusicKeyType};

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
        unimplemented!();
    }
}

//pub const FIRST:   ScaledValue = ScaledValue { index: 0, offset: 0 };
//pub const SECOND:  ScaledValue = ScaledValue { index: 1, offset: 0 };
//pub const THIRD:   ScaledValue = ScaledValue { index: 2, offset: 0 };
//pub const FOURTH:  ScaledValue = ScaledValue { index: 3, offset: 0 };
//pub const FIFTH:   ScaledValue = ScaledValue { index: 4, offset: 0 };
//pub const SIXTH:   ScaledValue = ScaledValue { index: 5, offset: 0 };
//pub const SEVENTH: ScaledValue = ScaledValue { index: 6, offset: 0 };
