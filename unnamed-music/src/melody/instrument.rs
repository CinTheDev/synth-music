use std::time::Duration;

pub struct ToneInfo {
    pub frequency: f64,
    pub time: Duration,
}

pub trait Instrument {
    fn generate_sound(&self, info: ToneInfo) -> f32;
}
