pub mod tet12;

use tet12::TET12ConcreteTone;
use super::Instrument;
use super::Tone;

/*

use std::time::Duration;

#[derive(Clone, Copy)]
struct SineGenerator;

#[derive(Clone, Copy)]
struct TriangleGenerator;

impl SineGenerator {
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl TriangleGenerator {
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        let x = time.as_secs_f64() * frequency * 2.0 * PI;
        return x.sin().asin() as f32;
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: std::time::Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result * info.intensity.start;
    }
}

impl Instrument for TriangleGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: std::time::Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result * info.intensity.start;
    }
}
*/
