use super::music_key::MusicKey;

pub trait ScaledValue {
    type ConcreteValue;

    fn to_concrete_value(&self, key: MusicKey) -> Self::ConcreteValue;
}

#[derive(Clone, Copy)]
pub enum Length {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
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

    pub dotted: bool,
    pub triole: bool,
}

impl<T: ScaledValue> Default for Note<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            length: Length::Quarter,
            play_fraction: 1.0,

            intensity: 1.0,
            dynamics_flag: DynamicsFlag::None,

            dotted: false,
            triole: false
        }
    }
}

impl<T: ScaledValue> Note<T> {
    pub fn staccato(&mut self) -> &mut Self {
        self.play_fraction = 0.2;
        self
    }

    pub fn dotted(&mut self) -> &mut Self {
        self.dotted = true;
        self
    }

    pub fn triole(&mut self) -> &mut Self {
        self.triole = true;
        self
    }

    //pub fn start_dynamic_change(&mut self) -> &mut Self {
    //    self.dynamics_flag = DynamicsFlag::StartChange;
    //    self
    //}

    //pub fn end_dynamic_change(&mut self, intensity: f32) -> &mut Self {
    //    self.dynamics_flag = DynamicsFlag::EndChange;
    //    self.intensity = intensity;
    //    self
    //}

    pub fn get_duration(&self, bpm: f32) -> std::time::Duration {
        let quarters_per_second = bpm / 60.0;
        let multiplier = self.length.get_time_length();
        let time = (4.0 * multiplier) / quarters_per_second;

        if self.dotted {
            return std::time::Duration::from_secs_f32(time * 1.5);
        }
        if self.triole {
            return std::time::Duration::from_secs_f32(time * 2.0 / 3.0);
        }

        return std::time::Duration::from_secs_f32(time);
    }
}

impl Length {
    fn get_time_length(self) -> f32 {
        let factor = match self {
            Length::Whole => 0,
            Length::Half => 1,
            Length::Quarter => 2,
            Length::Eigth => 3,
            Length::Sixteenth => 4,
        };

        return 1.0 / 2_f32.powi(factor);
    }
}
