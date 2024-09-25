
#[derive(Clone, Copy)]
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
    pub values: Vec<Tone>,
    pub length: Length,
    pub play_fraction: f32,
    pub intensity: f32,
}

impl Note {
    pub fn staccato(&mut self) {
        self.play_fraction = 0.1;
    }
}

/*
pub struct Note {
    pub tones: Vec<f32>,
    pub length: f32,
    pub intensity: f32,
}


impl Note {
    pub fn new(length: Length) -> Self {
        Self {
            tones: Vec::new(),
            length: length.get_time_length(),
            intensity: 1.0,
        }
    }

    pub fn tone(mut self, tone: Tone, octave_shift: i32) -> Self {
        let frequency = Self::get_tone_frequency(tone) * 2_f32.powi(octave_shift);
        self.tones.push(frequency);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn dotted(mut self) -> Self {
        self.length *= 1.5;
        self
    }

    fn get_tone_frequency(tone: Tone) -> f32 {
        match tone {
            Tone::C => Self::get_frequency_from_a4(-9),
            Tone::D => Self::get_frequency_from_a4(-7),
            Tone::E => Self::get_frequency_from_a4(-5),
            Tone::F => Self::get_frequency_from_a4(-4),
            Tone::G => Self::get_frequency_from_a4(-2),
            Tone::A => Self::get_frequency_from_a4(0),
            Tone::B => Self::get_frequency_from_a4(2),
        }
    }

    fn get_frequency_from_a4(semitones: i32) -> f32 {
        2_f32.powf(semitones as f32 / 12.0) * 440.0
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
*/
