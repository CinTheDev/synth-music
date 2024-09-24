
#[derive(Clone, Copy)]
pub enum Tone {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
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
    pub tones: Vec<f32>,
    pub length: Length,
    pub intensity: f32,
}


impl Note {
    pub fn new(length: Length) -> Self {
        Self {
            tones: Vec::new(),
            length: length,
            intensity: 1.0,
        }
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        let frequency = Self::get_tone_frequency(tone);
        self.tones.push(frequency);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
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
