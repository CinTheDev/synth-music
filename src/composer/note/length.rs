
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

    pub const fn dot(mut self) -> Self {
        self.ticks += self.ticks / 2;        
        self
    }

    pub const fn multi_dot(mut self, dots: usize) -> Self {
        let mut i = 0;
        let mut dot_value = self.ticks;

        while i < dots {
            dot_value /= 2;
            self.ticks += dot_value;

            i += 1;
        }

        self
    }

    pub const fn triole(self) -> Self {
        self.ntole(3)
    }

    pub const fn ntole(mut self, n: usize) -> Self {
        if n % 2 == 0 {
            panic!("Invalid n-tole");
        }

        let x = ((n - 1) / 2) as u8;

        self.ntole_index = x;
        self
    }

    pub fn to_float(&self) -> f32 {
        self.ticks as f32 / 256.0
    }
}
