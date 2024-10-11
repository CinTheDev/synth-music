use std::time::Duration;

use synth_music::prelude::*;

fn main() {
    println!("Intensity example");

    let linear_sine = LinearSine;
    let punchy_sine = PunchySine {
        cutoff_time: Duration::from_secs_f32(3.0),
    };

    let track_linear_sine = example_track(linear_sine);
    let track_punchy_sine = example_track(punchy_sine);

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },

        settings: &settings,
    };

    let linear_section = section!(section_info, track_linear_sine);
    let punchy_section = section!(section_info, track_punchy_sine);

    let composition = composition!(
        linear_section,
        punchy_section,
    );

    // Export
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let path = PathBuf::from("export/dynamics.wav");
    let exporter = WavExport {
        path,
        ..Default::default()
    };
    exporter.export(composition).unwrap();
}

fn example_track<T>(instrument: T) -> MeasureTrack<tet12::TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = tet12::TET12ConcreteTone>
{
    use note::length::*;
    use tet12::*;
    let mut track = MeasureTrack::new(instrument, time_signature::FOUR_FOUR);

    track.set_play_fraction(0.9);

    // Held notes

    track.set_intensity(0.3);

    track.start_dynamic_change();
    track.note(WHOLE, first(4));
    track.measure().unwrap();
    track.note(WHOLE, first(4));
    track.end_dynamic_change(1.0);
    track.measure().unwrap();
    
    track.start_dynamic_change();
    track.note(WHOLE, first(4));
    track.measure().unwrap();
    track.note(WHOLE, first(4));
    track.end_dynamic_change(0.3);
    track.measure().unwrap();

    track.pause(WHOLE);
    track.measure().unwrap();
    
    // Short notes

    track.start_dynamic_change();
    track.note(QUARTER, first(4));
    for _ in 0..2 {
        track.note(QUARTER, first(4));
    }
    track.note(QUARTER, first(4));
    track.end_dynamic_change(1.0);
    track.measure().unwrap();

    track.start_dynamic_change();
    track.note(QUARTER, first(4));
    for _ in 0..2 {
        track.note(QUARTER, first(4));
    }
    track.note(QUARTER, first(4));
    track.end_dynamic_change(0.3);
    track.measure().unwrap();

    track.pause(WHOLE);
    track.measure().unwrap();
    
    return track;
}

#[derive(Clone, Copy)]
struct LinearSine;

#[derive(Clone, Copy)]
struct PunchySine {
    cutoff_time: Duration,
}

impl LinearSine {
    fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result * Self::current_intensity(
            time.as_secs_f32(),
            tones.tone_duration.as_secs_f32(),
            &tones.intensity
        );
    }

    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn current_intensity(current_secs: f32, total_secs: f32, intensity: &std::ops::Range<f32>) -> f32 {
        let t =  current_secs / total_secs;
        return (intensity.start) + (intensity.end - intensity.start) * t;
    }
}

impl PunchySine {
    fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::wave(frequency, time);
        }

        return result * Self::decay(time.as_secs_f32()) * tones.intensity.start;
    }

    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn decay(secs: f32) -> f32 {
        0.5_f32.powf(secs * 2.0)
    }

    fn total_samples(&self, sample_rate: u32) -> usize {
        (sample_rate as f64 * self.cutoff_time.as_secs_f64()).ceil() as usize
    }
}

impl Instrument for LinearSine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        return InstrumentBuffer { samples: buffer }
    }
}

impl Instrument for PunchySine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        let samples = self.total_samples(buffer_info.sample_rate);

        for i in 0..samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        return InstrumentBuffer { samples: buffer }
    }
}
