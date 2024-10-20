use synth_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

pub mod drumset;
use drumset::DrumsetAction;

#[derive(Clone, Copy)]
pub struct SoftBass {
    pub decay_speed: f32,
}
    
#[derive(Clone, Copy)]
pub struct HardBass {
    pub harmonics: u32,
}

#[derive(Clone, Copy)]
pub struct Drumset {

}

impl SoftBass {
    pub fn new(decay_speed: f32) -> Self {
        Self {
            decay_speed,
        }
    }

    fn generate(&self, tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::generate_frequency(frequency, time);
        }

        return result * self.decay_function(time);
    }

    fn generate_frequency(frequency: f64, time: Duration) -> f32 {
        Self::triangle_wave(frequency, time)
    }

    fn triangle_wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        let x = time.as_secs_f64() * frequency * 2.0 * PI;
        x.sin().asin() as f32
    }

    fn decay_function(&self, time: Duration) -> f32 {
        0.5_f32.powf(time.as_secs_f32() * self.decay_speed)
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    fn generate(&self, tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += self.generate_frequency(frequency, time);
        }

        return result * tones.intensity.start;
    }

    fn generate_frequency(&self, frequency: f64, time: Duration) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, frequency, time);
        }

        return amplitude;
    }

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, frequency: f64, time: Duration) -> f32 {
        let factor = (2 * n + 1) as f32;
        Self::sine_wave(time, frequency * factor as f64) / factor
    }
}

// TODO: Improve this (make the DrumsetAction actually matter)
impl Drumset {
    fn random() -> f32 {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        rng.gen_range(-1.0..1.0)
    }

    fn frequency_range(action: DrumsetAction) -> (f32, f32) {
        match action {
            DrumsetAction::Bass => (0.0, 100.0),
            DrumsetAction::Snare => (100.0, 200.0),
            DrumsetAction::HiHat => (1000.0, 2000.0),
        }
    }

    fn generate(&self, time: Duration) -> f32 {
        if time > Duration::from_millis(100) {
            return 0.0;
        }

        let value = Self::random();
        return value;
    }

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Instrument for SoftBass {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(self.generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}

impl Instrument for HardBass {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(self.generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}

impl Instrument for Drumset {
    type ConcreteValue = DrumsetAction;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            let value = self.generate(time) * tones.intensity.start;
            buffer.push(value);
        }

        InstrumentBuffer { samples: buffer }
    }
}
