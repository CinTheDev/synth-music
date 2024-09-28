use std::time::Duration;
use dyn_clone::DynClone;

#[derive(Clone, Copy)]
pub struct ToneInfo {
    pub frequency: f64,
    pub time: Duration,
}

pub trait Instrument : DynClone {
    fn generate_sound(&self, info: ToneInfo) -> f32;
}

dyn_clone::clone_trait_object!(Instrument);
