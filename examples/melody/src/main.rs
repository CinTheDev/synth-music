use instrument::{note::Note, Track};
use unnamed_music::melody::*;

fn main() {
    println!("Melody Example");

    let melody_1 = melody_1();

    let debug_instrument = instrument::Instrument {
        tracks: vec![
            melody_1,
        ]
    };

    let first_section = Section {
        bpm: 120,
        instruments: vec![
            debug_instrument,
        ]
    };

    let piece = MusicPiece {
        sections: vec![
            first_section,
        ]
    };
}

fn melody_1() -> Track {
    use instrument::note::{Length, Tone};

    let mut track = Track::new();
    track.note(
        Note::new(Length::Whole)
            .tone(Tone::DbgA)
    );

    track.note(
        Note::new(Length::Whole)
            .tone(Tone::DbgB)
            .intensity(0.7)
    );

    return track;
}
