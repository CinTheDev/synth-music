use std::time::Duration;

pub trait Instrument {
    fn generate_sound(&self, frequency: f64, time: Duration) -> f32;
}
