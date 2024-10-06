pub mod predefined;

use crate::file_export::export_info::Tone;

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn generate_sound(&self, buffer: &mut [f32], info: &Tone<Self::ConcreteValue>);
}
