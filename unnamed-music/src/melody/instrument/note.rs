
pub enum Tone {
    DbgA,
    DbgB,
}

pub enum Length {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
}

pub struct Note {
    tones: Vec<Tone>,
    length: Length,
    intensity: f32,
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
        self.tones.push(tone);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }
}
