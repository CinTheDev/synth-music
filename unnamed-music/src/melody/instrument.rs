pub mod predefined;

use std::time::Duration;

//use crate::melody::export_info::TET12ConcreteValue;

#[derive(Clone, Copy)]
pub struct ToneInfo<T> {
    pub tone: T,
    pub time: Duration,
    pub intensity: f32,
}

pub trait Instrument<T> {
    fn generate_sound(&self, info: ToneInfo<T>) -> f32;
}
