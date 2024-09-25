use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let track_1 = track_1();

    let instrument_1 = Instrument {
        tracks: vec![track_1],
    };

    let section_1 = Section {
        bpm: 120.0,
        key: MusicKey::C,
        time_signature: (4, 4),

        instruments: vec![instrument_1],
    };

    let composition = Composition {
        sections: vec![section_1],
    };

    // TODO: More complex melody

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_1() -> Track {
    use note::Tone::*;
    let mut track = Track::new();

    track.note(First);
    track.note(Second);
    track.note(Third);
    track.note(Fourth);
    track.note(Fith);
    track.note(Sixth);
    track.note(Seventh);

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
