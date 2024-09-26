use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let instrument_1 = Instrument {
        tracks: vec![track_1()],
    };
    let instrument_2 = Instrument {
        tracks: vec![track_2()],
    };

    let section_1 = Section {
        bpm: 120.0,
        key: MusicKey::C,
        time_signature: (4, 4),

        instruments: vec![instrument_1],
    };
    let section_2 = Section {
        bpm: 120.0,
        key: MusicKey::C,
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

    track.note(Quarter, First);
    track.note(Quarter, Second);
    track.note(Quarter, Third);
    track.note(Quarter, Fourth);
    track.note(Quarter, Fith).staccato();
    track.note(Quarter, Sixth);
    track.note(Quarter, Seventh);

    return track;
}

fn track_2() -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new();

    track.note(Quarter, Sixth);
    track.note(Eigth, Seventh);
    track.note(Quarter, First);
    track.note(Eigth, Sixth);
    track.note(Eigth, First);
    track.note(Eigth, First);
    track.note(Eigth, Seventh);
    track.note(Eigth, Sixth);
    track.note(Quarter, Seventh);
    track.note(Quarter, Third);

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
