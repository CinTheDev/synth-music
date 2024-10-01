pub mod predefined;

use std::time::Duration;

#[derive(Clone, Copy)]
pub struct ToneInfo<T> {
    pub tone: T,
    pub time: Duration,
    pub intensity: f32,
}

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn generate_sound(&self, info: ToneInfo<Self::ConcreteValue>) -> f32;
}
