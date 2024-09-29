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

    pub fn get_concrete_value(self, key: crate::melody::composer::MusicKey) -> i32 {
        unimplemented!();
    }
}

#[derive(Clone, Copy)]
pub enum Length {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
}

//pub const FIRST:   ScaledValue = ScaledValue { index: 0, offset: 0 };
//pub const SECOND:  ScaledValue = ScaledValue { index: 1, offset: 0 };
//pub const THIRD:   ScaledValue = ScaledValue { index: 2, offset: 0 };
//pub const FOURTH:  ScaledValue = ScaledValue { index: 3, offset: 0 };
//pub const FIFTH:   ScaledValue = ScaledValue { index: 4, offset: 0 };
//pub const SIXTH:   ScaledValue = ScaledValue { index: 5, offset: 0 };
//pub const SEVENTH: ScaledValue = ScaledValue { index: 6, offset: 0 };

// Abstract note
#[derive(Clone)]
pub struct Note {
    pub values: Vec<(ScaledValue, i32)>,
    pub length: Length,
    pub play_fraction: f32,
    pub intensity: f32,

    pub dotted: bool,
    pub triole: bool,
}

// OLD CODE

impl Note {
    pub fn staccato(&mut self) -> &mut Self {
        self.play_fraction = 0.2;
        self
    }

    pub fn dotted(&mut self) -> &mut Self {
        self.dotted = true;
        self
    }

    pub fn triole(&mut self) -> &mut Self {
        self.triole = true;
        self
    }

    pub fn get_duration(&self, bpm: f32) -> std::time::Duration {
        let quarters_per_second = bpm / 60.0;
        let multiplier = self.length.get_time_length();
        let time = (4.0 * multiplier) / quarters_per_second;

        if self.dotted {
            return std::time::Duration::from_secs_f32(time * 1.5);
        }
        if self.triole {
            return std::time::Duration::from_secs_f32(time * 2.0 / 3.0);
        }

        return std::time::Duration::from_secs_f32(time);
    }
}

impl Length {
    fn get_time_length(self) -> f32 {
        let factor = match self {
            Length::Whole => 0,
            Length::Half => 1,
            Length::Quarter => 2,
            Length::Eigth => 3,
            Length::Sixteenth => 4,
        };

        return 1.0 / 2_f32.powi(factor);
    }
}
