use std::time::Duration;

use crate::melody::export_info::ConcreteValue;

#[derive(Clone, Copy)]
pub struct ToneInfo {
    pub tone: ConcreteValue,
    pub time: Duration,
    pub intensity: f32,
}

pub trait Instrument {
    fn generate_sound(&self, info: ToneInfo) -> f32;
}
