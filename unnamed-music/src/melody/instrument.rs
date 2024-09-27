use std::time::Duration;

pub trait Instrument {
    fn generate_sound(frequency: f32, time: Duration) -> f32;
}
