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
        unimplemented!();
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
