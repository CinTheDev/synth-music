
#[derive(Clone, Copy, Debug)]
pub enum Tone {
    First,
    Second,
    Third,
    Fourth,
    Fith,
    Sixth,
    Seventh,
}

#[derive(Clone, Copy)]
pub enum Length {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
}

pub struct Note {
    pub values: Vec<(Tone, i32)>,
    pub length: Length,
    pub play_fraction: f32,
    pub intensity: f32,

    pub semitones_offset: i32,

    pub dotted: bool,
    pub triole: bool,
}

impl Note {
    pub fn staccato(&mut self) {
        self.play_fraction = 0.2;
    }

    pub fn dotted(&mut self) {
        self.dotted = true;
    }

    pub fn triole(&mut self) {
        self.triole = true;
    }

    pub fn sharp(&mut self) {
        self.semitones_offset += 1;
    }

    pub fn flat(&mut self) {
        self.semitones_offset -= 1;
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
