
pub enum Tone {
    
}

pub enum Length {
    WHOLE,
    HALF,
    QUARTER,
    EIGTH,
    SIXTEENTH,
}

pub struct Note {
    tones: Vec<Tone>,
    length: Length,
    intensity: f32,
}
