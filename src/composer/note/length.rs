
pub const WHOLE:     Length = Length::from_subdivisions(0);
pub const HALF:      Length = Length::from_subdivisions(1);
pub const QUARTER:   Length = Length::from_subdivisions(2);
pub const EIGTH:     Length = Length::from_subdivisions(3);
pub const SIXTEENTH: Length = Length::from_subdivisions(4);

pub const ZERO: Length = Length::from_ticks(0);

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

    pub fn count_lengths(lengths: &Vec<Self>) -> Result<Self, &str> {
        let mut total_length = ZERO;

        let mut ntole_count: Vec<(Self, u16)> = Vec::new();

        for length in lengths {
            if length.ntole_index == 0 {
                total_length.ticks += length.ticks;
            }
            else {
                total_length.ticks += Self::count_ntole(&mut ntole_count, *length);
            }
        }

        if ! ntole_count.is_empty() {
            return Err("Not all ntoles have simplified");
        }

        Ok(total_length)
    }

    fn ntole_parts_from_index(ntole_index: u8) -> u16 {
        ntole_index as u16 * 2 + 1
    }

    fn count_ntole(ntole_count: &mut Vec<(Self, u16)>, length: Self) -> u32 {
        // Search if ntole index exists in vector
        for i in 0..ntole_count.len() {
            if ntole_count[i].0 != length {
                continue;
            }

            // It does exist

            ntole_count[i].1 -= 1;

            if ntole_count[i].1 > 0 {
                return 0;
            }

            // ntole simplifies to note
            ntole_count.remove(i);
            return length.ticks * 2;
        }

        let expected_ntole_parts = Self::ntole_parts_from_index(length.ntole_index);
        ntole_count.push((length, expected_ntole_parts - 1));

        return 0;
    }
}

impl std::ops::Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        if self.ntole_index != rhs.ntole_index {
            panic!("Adding two different ntoles is not supported.");
        }

        Self {
            ticks: self.ticks + rhs.ticks,
            ntole_index: self.ntole_index,
        }
    }
}
impl std::ops::AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        if self.ntole_index != rhs.ntole_index {
            panic!("Adding two different ntoles is not supported.");
        }

        self.ticks += rhs.ticks;
    }
}

impl std::ops::Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.ntole_index != rhs.ntole_index {
            panic!("Subtracting two different ntoles is not supported.");
        }

        Self {
            ticks: self.ticks - rhs.ticks,
            ntole_index: self.ntole_index,
        }
    }
}

impl std::ops::SubAssign for Length {
    fn sub_assign(&mut self, rhs: Self) {
        if self.ntole_index != rhs.ntole_index {
            panic!("Subtracting two different ntoles is not supported.");
        }

        self.ticks -= rhs.ticks;
    }
}

impl std::ops::Mul<u32> for Length {
    type Output = Length;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            ticks: self.ticks * rhs,
            ntole_index: self.ntole_index,
        }
    }
}
impl std::ops::MulAssign<u32> for Length {
    fn mul_assign(&mut self, rhs: u32) {
        self.ticks *= rhs;
    }
}
