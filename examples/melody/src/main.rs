use instrument::{note::Note, Track};
use unnamed_music::melody::*;

fn main() {
    println!("Melody Example");

    let debug_instrument = instrument::Instrument {
        tracks: vec![
            melody_1(),
        ]
    };

    let second_instrument = instrument::Instrument {
        tracks: vec![
            melody_2(),
        ],
    };

    let first_section = Section {
        bpm: 120,
        instruments: vec![
            debug_instrument,
        ]
    };

    let second_section = Section {
        bpm: 120,
        instruments: vec![
            second_instrument
        ],
    };

    let piece = MusicPiece {
        sections: vec![
            first_section,
            second_section,
        ]
    };

    export_piece(piece);
}

fn melody_1() -> Track {
    use instrument::note::{Length, Tone};

    let mut track = Track::new();

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::C, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::D, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::E, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::F, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::G, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::A, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::B, 0)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::C, 1)
    );
    
    return track;
}

fn melody_2() -> Track {
    use instrument::note::{Length, Tone};

    let mut track = Track::new();

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::A, -1)
            .dotted()
    );

    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::B, -1)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::C, 0)
            .dotted()
    );

    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::A, -1)
    );

    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::C, 0)
    );
    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::C, 0)
    );
    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::B, -1)
    );
    track.note(
        Note::new(Length::Eigth)
            .tone(Tone::A, -1)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::B, -1)
    );

    track.note(
        Note::new(Length::Quarter)
            .tone(Tone::E, -1)
    );

    return track;
}

fn export_piece(piece: MusicPiece) {
    use unnamed_music::file_export::*;
    use unnamed_music::file_export::wav_export::WavExport;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let music_buffer = MusicBuffer::new(piece);
    let exporter = WavExport {
        path: std::path::PathBuf::from("export/debug.wav"),
        sample_rate: 44100,
        bits_per_sample: 16,
    };

    exporter.export(music_buffer).unwrap();
}
