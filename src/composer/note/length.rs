
pub const WHOLE:     Length = Length::new(256*256);
pub const HALF:      Length = Length::new(256*256 / 2);
pub const QUARTER:   Length = Length::new(256*256 / 4);
pub const EIGTH:     Length = Length::new(256*256 / 8);
pub const SIXTEENTH: Length = Length::new(256*256 / 16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Length {
    // Number of 256th values
    ticks: u32,
    ntole_index: u8,
}

impl Length {
    pub const fn new(ticks: u32) -> Self {
        Self {
            ticks,
            ntole_index: 0,
        }
    }
}
