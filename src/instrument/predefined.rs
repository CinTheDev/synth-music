pub mod tet12;

use tet12::TET12ConcreteTone;
use super::Instrument;

use std::time::Duration;

/// Return a point of a sine wave given a frequency and time
pub fn sine_wave(frequency: f64, time: Duration) -> f32 {
    use std::f64::consts::PI;
    return (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32;
}

/// Return a point of a square wave given a frequency and time
pub fn square_wave(frequency: f64, time: Duration) -> f32 {
    let x = (2.0 * time.as_secs_f64() * frequency + 1.0).floor() as u32;
    return 2.0 * (x % 2) as f32 - 1.0;
}

/// Return a point of a triangle wave given a frequency and time
pub fn triangle_wave(frequency: f64, time: Duration) -> f32 {
    use std::f64::consts::PI;
    let x = time.as_secs_f64() * frequency * 2.0 * PI;
    return ((2.0 / PI) * x.sin().asin()) as f32;
}

/// Return a point of a saw wave given a frequency and time
pub fn saw_wave(frequency: f64, time: Duration) -> f32 {
    let x = (2.0 * time.as_secs_f64() * frequency + 1.0) as f32;
    return (x % 2.0) - 1.0;
}

/// An implementor for `Instrument` that uses a raw sine wave.
#[derive(Clone, Copy)]
pub struct SineGenerator;

/// An implementor for `Instrument` that uses a raw triangle wave.
#[derive(Clone, Copy)]
pub struct TriangleGenerator;

/// An implementor for `Instrument` that uses a raw square wave.
#[derive(Clone, Copy)]
pub struct SquareGenerator;

/// An implementor for `Instrument` that uses a raw saw wave.
#[derive(Clone, Copy)]
pub struct SawGenerator;

impl Instrument for SineGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return sine_wave(frequency, time);
    }
}

impl Instrument for TriangleGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return triangle_wave(frequency, time);
    }
}

impl Instrument for SquareGenerator {
    type ConcreteValue = TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return square_wave(frequency, time);
    }
}

impl Instrument for SawGenerator {
    type ConcreteValue = TET12ConcreteTone;
    
    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return saw_wave(frequency, time);
    }
}
