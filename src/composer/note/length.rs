
pub const WHOLE:     Length = Length::from_subdivisions(0);
pub const HALF:      Length = Length::from_subdivisions(1);
pub const QUARTER:   Length = Length::from_subdivisions(2);
pub const EIGTH:     Length = Length::from_subdivisions(3);
pub const SIXTEENTH: Length = Length::from_subdivisions(4);

pub const ZERO: Length = Length::from_ticks(0);

const TICKS_WHOLE: u32 = 2_u32.pow(16);

/// Represents a note length. It can take many forms such as dotted notes and
/// also n-toles.
/// 
/// It's best to use the provided constants, it should only rarely be necessary
/// to use the constructors.
/// 
/// ```
/// # use synth_music::prelude::*;
/// # use note::length::*;
/// # use tet12::*;
/// let dotted_quarter = QUARTER.dot();
/// let eighth_triole = EIGTH.triole();
/// 
/// let mut track = UnboundTrack::new(predefined::SineGenerator);
/// 
/// track.note(HALF, first(4));
/// 
/// track.pause(SIXTEENTH.triole());
/// track.pause(SIXTEENTH.triole());
/// track.pause(SIXTEENTH.triole());
/// 
/// track.note(EIGTH.dot(), third(3));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Length {
    // Number of 65536th values
    ticks: u32,
    ntole_index: u8,
}

impl Length {
    pub const INVALID: Self = Self::from_ticks(u32::MAX);
    pub const ZERO: Self = Self::from_ticks(0);

    /// Construct a length from internally used "ticks". A "tick" is a 65536th
    /// note, and therefore the shortest representable note.
    pub const fn from_ticks(ticks: u32) -> Self {
        Self {
            ticks,
            ntole_index: 0,
        }
    }

    /// Construct a length by subdividing a Whole note. The provided constants
    /// already do this.
    /// 
    /// A subdivision of `0` is a whole note, `1` is half, `2` is a quarter,
    /// etc...
    pub const fn from_subdivisions(subdivision: u32) -> Self {
        let ticks = TICKS_WHOLE / 2_u32.pow(subdivision);
        Self::from_ticks(ticks)
    }

    /// Extend the note length by half of its value. Represents a "dot" after
    /// the note in music theory.
    /// 
    /// For multiple dots, `multi_dot()` will have the same behaviour as in
    /// music theory. Refrain from calling `dot()` multiple times for
    /// this.
    pub const fn dot(mut self) -> Self {
        self.ticks += self.ticks / 2;        
        self
    }

    /// Place a specified amount of dots on a note. Every dot will extend the
    /// length by half of the previous extention. E.g. two dots on a whole note
    /// will extend it by a half and a quarter note.
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

    /// Mark the note length as being inside a triole. There should be two other
    /// notes in the same measure with the same length and triole specifier for
    /// this to be valid.
    /// 
    /// All three note lengths will combine to the double of the base length;
    /// e.g. three notes in a quarter triole are equivalent to a half note
    /// length.
    pub const fn triole(self) -> Self {
        self.ntole(3)
    }

    /// Mark the note length as being inside an n-tole. There should be a total
    /// of n notes in the same measure with the same length and n-tole specifier
    /// for this to be valid.
    /// 
    /// All note lengths will combine to the double of the base length; e.g.
    /// n notes in a quarter triole are equivalent to a half note length.
    /// 
    /// Currently, only odd values for n are accepted (3, 5, 7, etc...)
    pub const fn ntole(mut self, n: usize) -> Self {
        if n % 2 == 0 {
            panic!("Invalid n-tole");
        }

        let x = ((n - 1) / 2) as u8;

        self.ntole_index = x;
        self
    }

    /// Convert the note length into a float. This can be imprecise and
    /// shouldn't be used for comparing note lenghts (with `==` or `!=`).
    pub fn to_float(&self) -> f32 {
        let base_length = self.ticks as f32 / TICKS_WHOLE as f32;

        let ntole_multiplier = if self.ntole_index == 0 {
            1.0
        } else {
            2.0 / Self::ntole_parts_from_index(self.ntole_index) as f32
        };

        return base_length * ntole_multiplier;
    }

    /// For precicesly combining multiple note lengths into one.
    /// 
    /// Will return an error if there are incomplete n-toles.
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
