pub mod tet12;

use tet12::TET12ConcreteTone;
use super::Instrument;
use super::Tone;
use crate::file_export::export_info::SoundBuffer;

use std::time::Duration;

#[derive(Clone, Copy)]
struct SineGenerator;

#[derive(Clone, Copy)]
struct TriangleGenerator;

impl SineGenerator {
    pub fn generate(info: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result * info.intensity.start;
    }

    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl TriangleGenerator {
    pub fn generate(info: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result;
    }

    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        let x = time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin().asin() as f32;
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, buffer: &mut SoundBuffer, info: &Tone<Self::ConcreteValue>) {
        for i in 0..buffer.samples.len() {
            let time = buffer.get_time_from_index(i);
            buffer.samples[i] = Self::generate(info, time);
        }
    }
}

impl Instrument for TriangleGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, buffer: &mut SoundBuffer, info: &Tone<Self::ConcreteValue>) {
        for i in 0..buffer.samples.len() {
            let time = buffer.get_time_from_index(i);
            buffer.samples[i] = Self::generate(info, time);
        }
    }
}
