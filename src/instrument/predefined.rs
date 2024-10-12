pub mod tet12;

use tet12::TET12ConcreteTone;
use super::Instrument;
use super::Tone;
use super::InstrumentBuffer;

use std::time::Duration;

pub fn sine_wave(frequency: f64, time: Duration) -> f32 {
    use std::f64::consts::PI;
    return (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32;
}

pub fn square_wave(frequency: f64, time: Duration) -> f32 {
    let x = (2.0 * time.as_secs_f64() * frequency + 1.0).floor() as u32;
    return 2.0 * (x % 2) as f32 - 1.0;
}

pub fn triangle_wave(frequency: f64, time: Duration) -> f32 {
    use std::f64::consts::PI;
    let x = time.as_secs_f64() * frequency * 2.0 * PI;
    return ((2.0 / PI) * x.sin().asin()) as f32;
}

pub fn saw_wave(frequency: f64, time: Duration) -> f32 {
    let x = (2.0 * time.as_secs_f64() * frequency + 1.0) as f32;
    return (x % 2.0) - 1.0;
}

#[derive(Clone, Copy)]
pub struct SineGenerator;

#[derive(Clone, Copy)]
pub struct TriangleGenerator;

#[derive(Clone, Copy)]
pub struct SquareGenerator;

impl SineGenerator {
    pub fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += sine_wave(frequency, time);
        }

        return result * tones.intensity.start;
    }
}

impl TriangleGenerator {
    pub fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += triangle_wave(frequency, time);
        }

        return result * tones.intensity.start;
    }
}

impl SquareGenerator {
    pub fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += square_wave(frequency, time);
        }

        return result * tones.intensity.start;
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: super::BufferInfo, tones: &Tone<Self::ConcreteValue>) -> super::InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}

impl Instrument for TriangleGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: super::BufferInfo, tones: &Tone<Self::ConcreteValue>) -> super::InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}

impl Instrument for SquareGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: super::BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}
