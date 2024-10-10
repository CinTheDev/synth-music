use super::music_key::MusicKey;

pub mod length;
pub use length::Length;

pub trait ScaledValue: Clone {
    type ConcreteValue;

    fn to_concrete_value(&self, key: MusicKey) -> Self::ConcreteValue;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DynamicsFlag {
    None,
    StartChange,
    EndChange,
}

// Abstract note
#[derive(Clone)]
pub struct Note<T: ScaledValue> {
    pub values: Vec<T>,
    pub length: Length,
    pub play_fraction: f32,

    pub intensity: f32,
    pub dynamics_flag: DynamicsFlag,
}

impl<T: ScaledValue> Default for Note<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            length: length::QUARTER,
            play_fraction: 1.0,

            intensity: 1.0,
            dynamics_flag: DynamicsFlag::None,
        }
    }
}

impl<T: ScaledValue> Note<T> {
    pub fn staccato(&mut self) -> &mut Self {
        self.play_fraction = 0.2;
        self
    }

    pub fn get_duration(&self, bpm: f32) -> std::time::Duration {
        let quarters_per_second = bpm / 60.0;
        let note_length = self.length.to_float();
        let time = (4.0 * note_length) / quarters_per_second;

        return std::time::Duration::from_secs_f32(time);
    }
}
