
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
