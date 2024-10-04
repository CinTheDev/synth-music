use std::time::Duration;

use synth_music::prelude::*;

fn main() {
    println!("Intensity example");

    let linear_sine = LinearSine;
    let punchy_sine = PunchySine;

    let track_linear_sine = example_track(linear_sine);
    let track_punchy_sine = example_track(punchy_sine);

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },
        time_signature: (4, 4),
    };

    let mut linear_section = section!(section_info, 44100, track_linear_sine);
    let mut punchy_section = section!(section_info, 44100, track_punchy_sine);

    let mut composition = Vec::new();
    composition.append(&mut linear_section);
    composition.append(&mut punchy_section);

    // Export
    use std::path::PathBuf;

    if std::fs::remove_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let path = PathBuf::from("export/dynamics.wav");
    let exporter = WavExport {
        path,
        ..Default::default()
    };
    exporter.export(composition).unwrap();
}

fn example_track<T>(instrument: T) -> UnboundTrack<tet12::TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = tet12::TET12ConcreteTone>
{
    use note::Length::*;
    use note::DynamicsFlag;
    use tet12::*;
    let mut track = UnboundTrack::new(instrument);

    // Held notes

    track.set_intensity(0.3);

    track.note(Whole, first(4)).dynamics(DynamicsFlag::StartChange);
    track.note(Whole, first(4));
    track.note(Whole, first(4));
    
    track.set_intensity(1.0);
    track.note(Whole, first(1)).dynamics(DynamicsFlag::EndChange);

    track.note(Whole, first(4)).dynamics(DynamicsFlag::StartChange);
    track.note(Whole, first(4));
    track.note(Whole, first(4));

    track.set_intensity(0.3);
    track.note(Whole, first(4)).dynamics(DynamicsFlag::EndChange);

    // Short notes
    track.pause(Whole);

    track.note(Quarter, first(4)).dynamics(DynamicsFlag::StartChange);

    for _ in 0..1 * 4-2 {
        track.note(Quarter, first(4));
    }
    track.set_intensity(1.0);
    for _ in 0..1 * 4-2 {
        track.note(Quarter, first(4));
    }

    track.set_intensity(0.3);
    track.note(Quarter, first(4)).dynamics(DynamicsFlag::EndChange);
    
    return track;
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
