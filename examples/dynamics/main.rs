use std::time::Duration;

use synth_music::prelude::*;

fn main() {
    println!("Intensity example");

    let linear_sine = LinearSine;
    let punchy_sine = PunchySine {
        cutoff_time: Duration::from_secs_f32(3.0),
    };

    let track_constant_intensity = example_constant_intensity(linear_sine);
    let track_linear_sine = example_dynamic_intensity(linear_sine);
    let track_punchy_sine = example_dynamic_intensity(punchy_sine);

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

    let constant_section = section!(section_info, track_constant_intensity);
    let linear_section = section!(section_info, track_linear_sine);
    let punchy_section = section!(section_info, track_punchy_sine);

    let composition = composition!(
        constant_section,
        linear_section,
        punchy_section,
    );

    export(composition);
}

fn example_constant_intensity<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    use length::*;
    use tet12::*;

    let time_signature = TimeSignature::new(4, 4);

    let mut track = MeasureTrack::new(instrument, time_signature);

    // track.set_intensity(x) will change the intensity for the notes placed
    // after the call.

    track.set_play_fraction(0.9);

    track.set_intensity(0.1);
    track.note(QUARTER, first(4));
    track.set_intensity(0.2);
    track.note(QUARTER, first(4));
    track.set_intensity(0.3);
    track.note(QUARTER, first(4));
    track.set_intensity(0.4);
    track.note(QUARTER, first(4));

    track.measure().unwrap();

    track.set_intensity(0.5);
    track.note(QUARTER, first(4));
    track.set_intensity(0.4);
    track.note(QUARTER, first(4));
    track.set_intensity(0.3);
    track.note(QUARTER, first(4));
    track.set_intensity(0.2);
    track.note(QUARTER, first(4));

    track.measure().unwrap();

    track.pause(WHOLE);
    track.measure().unwrap();

    return track;
}

fn example_dynamic_intensity<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    use length::*;
    use tet12::*;

    let time_signature = TimeSignature::new(4, 4);

    let mut track = MeasureTrack::new(instrument, time_signature);

    track.set_play_fraction(0.9);

    // Held notes

    track.set_intensity(0.1);

    track.start_dynamic_change(); // Start changing loudness
    track.note(WHOLE, first(4));
    track.measure().unwrap();
    track.note(WHOLE, first(4));
    track.end_dynamic_change(0.7); // Loudness changing stops here at the given value
    track.measure().unwrap();
    
    track.start_dynamic_change();
    track.note(WHOLE, first(4));
    track.measure().unwrap();
    track.note(WHOLE, first(4));
    track.end_dynamic_change(0.1);
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
    track.end_dynamic_change(0.7);
    track.measure().unwrap();

    track.start_dynamic_change();
    track.note(QUARTER, first(4));
    for _ in 0..2 {
        track.note(QUARTER, first(4));
    }
    track.note(QUARTER, first(4));
    track.end_dynamic_change(0.1);
    track.measure().unwrap();

    track.pause(WHOLE);
    track.measure().unwrap();
    
    return track;
}

fn export(buffer: SoundBuffer) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    // Specify new exporter with given attributes
    let wav_export = WavExport {
        path: PathBuf::from("export/Dynamics.wav"),
        ..Default::default()
    };

    // Actually export the piece.
    wav_export.export(buffer).unwrap();
}

#[derive(Clone, Copy)]
struct LinearSine;

#[derive(Clone, Copy)]
struct PunchySine {
    cutoff_time: Duration,
}

impl LinearSine {
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl PunchySine {
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn decay(time: Duration) -> f32 {
        0.5_f32.powf(time.as_secs_f32() * 2.0)
    }
}

impl Instrument for LinearSine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return Self::wave(frequency, time);
    }

    // This is the default behaviour for this function, so it does not need to
    // be overriden. Now it is written out for clarity.
    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let t = time.as_secs_f32() / tones.play_duration.as_secs_f32();
        let intensity = &tones.intensity;

        return (intensity.end - intensity.start) * t + intensity.start;
    }
}

impl Instrument for PunchySine {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return Self::wave(frequency, time);
    }

    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let base = tones.intensity.start;
        let decay_factor = Self::decay(time);
        return base * decay_factor;
    }

    fn get_num_samples(&self, buffer_info: &SoundBuffer, _tones: &Tone<Self::ConcreteValue>) -> usize {
        let sample_rate = buffer_info.settings().sample_rate;
        return (sample_rate as f64 * self.cutoff_time.as_secs_f64()).ceil() as usize
    }
}
