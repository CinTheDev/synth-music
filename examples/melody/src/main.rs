use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let instrument_1 = Instrument {
        tracks: vec![track_1()],
    };
    let instrument_2 = Instrument {
        tracks: vec![track_2()],
    };

    let first_key = MusicKey {
        base: MusicKeyBase::C,
        key_type: MusicKeyType::Major,
    };
    let second_key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let section_1 = Section {
        bpm: 120.0,
        key: first_key,
        time_signature: (4, 4),

        instruments: vec![instrument_1],
    };
    let section_2 = Section {
        bpm: 120.0,
        key: second_key,
        time_signature: (4, 4),

        instruments: vec![instrument_2],
    };

    let composition = Composition {
        sections: vec![section_1, section_2],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_1() -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new();

    track.note(Quarter, First, 0);
    track.note(Quarter, Second, 0);
    track.note(Quarter, Third, 0);
    track.note(Quarter, Fourth, 0);
    track.note(Quarter, Fith, 0).staccato();
    track.note(Quarter, Sixth, 0);
    track.note(Quarter, Seventh, 0);

    return track;
}

fn track_2() -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new();

    track.note(Quarter, Sixth, 0);
    track.note(Eigth, Seventh, 0);
    track.note(Quarter, First, 1);
    track.note(Eigth, Sixth, 0);
    track.note(Eigth, First, 1);
    track.note(Eigth, First, 1);
    track.note(Eigth, Seventh, 0);
    track.note(Eigth, Sixth, 0);
    track.note(Quarter, Seventh, 0);
    track.note(Quarter, Third, 0);

    return track;
}

fn export(export_piece: ExportMusicPiece) {
    use unnamed_music::file_export::*;
    use wav_export::WavExport;
    use std::path::PathBuf;

    let music_buffer = MusicBuffer::new(export_piece);
    let exporter = WavExport {
        path: PathBuf::from("export/debug.wav"),
        sample_rate: 44100,
        ..Default::default()
    };

    exporter.export(music_buffer).unwrap();
}
