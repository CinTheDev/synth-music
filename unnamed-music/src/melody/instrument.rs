use std::time::Duration;
use dyn_clone::DynClone;

use crate::melody::export_info::ConcreteValue;

#[derive(Clone, Copy)]
pub struct ToneInfo {
    pub tone: ConcreteValue,
    pub time: Duration,
    pub intensity: f32,
}

pub trait Instrument : DynClone {
    fn generate_sound(&self, info: ToneInfo) -> f32;
}

dyn_clone::clone_trait_object!(Instrument);
