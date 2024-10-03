use synth_music::prelude::*;

#[derive(Clone, Copy)]
pub enum DrumsetAction {
    Bass,
    Snare,
    HiHat,
}

impl ScaledValue for DrumsetAction {
    type ConcreteValue = Self;

    fn to_concrete_value(&self, _key: MusicKey) -> Self::ConcreteValue {
        self.to_owned()
    }
}
