use std::time::Duration;

use synth_music::prelude::*;

fn main() {
    println!("Intensity example");
}

#[derive(Clone, Copy)]
struct LinearSine;

#[derive(Clone, Copy)]
struct PunchySine;

impl LinearSine {
    fn wave(frequency: f64, secs: f64) -> f32 {
        use std::f64::consts::PI;
        (secs * frequency * 2.0 * PI).sin() as f32
    }

    fn current_intensity(current_secs: f32, total_secs: f32) -> f32 {
        return current_secs / total_secs;
    }
}

impl PunchySine {
    fn wave(frequency: f64, secs: f64) -> f32 {
        use std::f64::consts::PI;
        (secs * frequency * 2.0 * PI).sin() as f32
    }

    fn decay(secs: f32) -> f32 {
        0.5_f32.powf(secs * 3.0)
    }
}

impl Instrument for LinearSine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            let wave = Self::wave(frequency, time.as_secs_f64());
            result += wave;
        }

        return result * Self::current_intensity(time.as_secs_f32(), info.tone_duration.as_secs_f32());
    }
}

impl Instrument for PunchySine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn generate_sound(&self, info: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            let wave = Self::wave(frequency, time.as_secs_f64());
            result += wave;
        }

        return result * Self::decay(time.as_secs_f32()) * info.intensity.start;
    }
}
