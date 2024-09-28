use std::time::Duration;

#[derive(Clone, Copy)]
pub struct ToneInfo {
    pub frequency: f64,
    pub time: Duration,
}

pub trait Instrument {
    fn generate_sound(&self, info: ToneInfo) -> f32;
}
