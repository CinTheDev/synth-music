
#[derive(Clone, Copy)]
pub enum Tone {
    DbgA,
    DbgB,
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
    pub tones: Vec<Tone>,
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
        self.tones.push(tone);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }
}
