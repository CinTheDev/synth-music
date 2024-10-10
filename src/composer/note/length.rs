
pub const WHOLE:     Length = Length::from_subdivisions(0);
pub const HALF:      Length = Length::from_subdivisions(1);
pub const QUARTER:   Length = Length::from_subdivisions(2);
pub const EIGTH:     Length = Length::from_subdivisions(3);
pub const SIXTEENTH: Length = Length::from_subdivisions(4);

const TICKS_WHOLE: u32 = 2_u32.pow(16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Length {
    // Number of 65536th values
    ticks: u32,
    ntole_index: u8,
}

impl Length {
    pub const fn from_ticks(ticks: u32) -> Self {
        Self {
            ticks,
            ntole_index: 0,
        }
    }

    pub const fn from_subdivisions(subdivision: u32) -> Self {
        let ticks = TICKS_WHOLE / 2_u32.pow(subdivision);
        Self::from_ticks(ticks)
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
        self.ticks as f32 / TICKS_WHOLE as f32
    }
}
