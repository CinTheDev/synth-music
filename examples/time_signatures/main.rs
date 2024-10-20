use synth_music::prelude::*;

fn main() {
    let four_four = track_four_four();
    let three_four = track_three_four();
    let six_eight = track_six_eight();

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key: music_key::A_MINOR,
        settings: &settings,
    };

    let section_four_four = section!(info, four_four);
    let section_three_four = section!(info, three_four);
    let section_six_eight = section!(info, six_eight);

    let composition = composition!(
        section_four_four,
        section_three_four,
        section_six_eight,
    );

    export(composition);
}

fn track_four_four() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    // Create a new time signature
    let time_signature =
        TimeSignature::new(4, 4) // The beat will be 4/4
        .set_beat(0, 1.1) // The first beat is emphasized strongly
        .set_beat(2 , 1.05); // The third beat is emphasized weakly

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);
    track.set_play_fraction(0.8);

    for _ in 0..4 {
        sequential_notes!(track, QUARTER,
            first(3),
            first(3),
            first(3),
            first(3),
        );
        track.measure().unwrap();
    }

    track.pause(WHOLE);
    track.measure().unwrap();

    return track;
}

fn track_three_four() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    // Create new time signature
    let time_signature =
        TimeSignature::new(3, 4) // The beat will be 3/4
        .set_beat(0, 1.1);  // The first beat is emphasized

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);
    track.set_play_fraction(0.8);

    for _ in 0..4 {
        sequential_notes!(track, QUARTER,
            third(3),
            third(3),
            third(3),
        );
        track.measure().unwrap();
    }

    track.pause(HALF.dot());
    track.measure().unwrap();

    return track;
}

fn track_six_eight() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    // Create new time signature
    let time_signature =
        TimeSignature::new(6, 8) // The beat will be 6/8
        .set_beat(0, 1.1) // The first beat is emphasized strongly
        .set_beat(3 , 1.05); // The fourth beat is emphasized weakly

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);
    track.set_play_fraction(0.8);

    for _ in 0..4 {
        sequential_notes!(track, EIGTH,
            first(3),
            first(3),
            first(3),

            third(3),
            third(3),
            third(3),
        );
        track.measure().unwrap();
    }

    track.pause(HALF.dot());
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
        path: PathBuf::from("export/TimeSignature.wav"),
        ..Default::default()
    };

    // Actually export the piece.
    wav_export.export(buffer).unwrap();
}
