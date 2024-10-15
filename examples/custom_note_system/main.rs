use synth_music::prelude::*;

mod drumset;
use drumset::{DrumsetAction, Drumset};

fn main() {
    let track = test_track();

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key: music_key::C_MAJOR,
        settings: &settings,
    };

    let section = section!(info, track);

    export(section);
}

fn test_track() -> MeasureTrack<DrumsetAction, Drumset> {
    use DrumsetAction::*;
    use length::*;

    let four_four = TimeSignature::new(4, 4);
    let drumset = Drumset::new();

    let mut track = MeasureTrack::new(drumset, four_four);

    for _ in 0..2 {
        track.note(QUARTER, Bass);
        track.note(QUARTER, Snare);
        track.note(QUARTER, Bass);
        track.note(QUARTER, Snare);
        track.measure().unwrap();
    }

    track.pause(WHOLE);
    track.measure().unwrap();

    for _ in 0..2 {
        notes!(track, QUARTER, Bass);
        notes!(track, QUARTER, Bass, Snare);
        notes!(track, QUARTER, Bass);
        notes!(track, QUARTER, Bass, Snare);
        track.measure().unwrap();
    }

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
        path: PathBuf::from("export/Drumset.wav"),
        ..Default::default()
    };

    // Actually export the piece.
    wav_export.export(buffer).unwrap();
}
