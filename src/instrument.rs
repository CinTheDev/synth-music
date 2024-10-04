pub mod predefined;

use std::time::Duration;

use crate::file_export::export_info::Tone;

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32;
}
