pub mod predefined;

use crate::file_export::export_info::{Tone, SoundBuffer};

pub trait Instrument: Clone {
    type ConcreteValue: Clone + Copy;

    fn generate_sound(&self, buffer: &mut SoundBuffer, info: &Tone<Self::ConcreteValue>);
}
