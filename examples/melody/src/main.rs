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
    // TODO: Export
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
