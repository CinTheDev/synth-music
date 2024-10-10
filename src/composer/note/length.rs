
pub struct Length {
    // Number of 256th values
    ticks: u32,
    ntole_index: u8,
}

impl Length {
    pub fn new(ticks: u32) -> Self {
        Self {
            ticks,
            ntole_index: 0,
        }
    }
}
