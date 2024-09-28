use unnamed_music::melody::prelude::*;

mod instruments;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key,
        time_signature: (4, 4),
    };

    let instrument_softbass = instruments::SoftBass::new(1.0);
    let instrument_hardbass = instruments::HardBass::new(10);

    let melody_begin = track_melody_begin(Box::new(instrument_softbass));
    let chords_begin = track_chords_begin(Box::new(instrument_softbass));
    let bass_begin = track_bass_begin(Box::new(instrument_hardbass));

    let melody_repeated_first = track_melody_repeated(Box::new(instrument_softbass), true);
    let melody_repeated_second = track_melody_repeated(Box::new(instrument_softbass), false);
    let chords_repeated = track_chords_repeated(Box::new(instrument_softbass));
    let bass_repeated = track_bass_repeated(Box::new(instrument_hardbass));

    let section_begin = Section {
        info,
        tracks: vec![
            melody_begin,
            chords_begin,
            bass_begin,
        ],
    };

    let section_repeated_first = Section {
        info,
        tracks: vec![
            melody_repeated_first,
            chords_repeated.clone(),
            bass_repeated.clone(),
        ],
    };

    let section_repeated_second = Section {
        info,
        tracks: vec![
            melody_repeated_second,
            chords_repeated.clone(),
            bass_repeated.clone(),
        ],
    };

    let composition = Composition {
        sections: vec![
            section_begin,
            section_repeated_first,
            section_repeated_second
        ],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_melody_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, Fith, 3);
    track.note(Eigth, Second, 3);
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fourth, 3);
    track.note(Eigth, Third, 3);
    track.note(Eigth, Second, 3);

    track.note(Quarter, First, 3);
    track.note(Eigth, First, 3);
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fith, 3);
    track.note(Eigth, Fourth, 3);
    track.note(Eigth, Third, 3);

    track.note(Quarter, Second, 3).dotted();
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fourth, 3);
    track.note(Quarter, Fith, 3);

    apply_melody_end_part(&mut track);

    return track;
}

fn track_chords_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.2);

    for _ in 0..2 {
        // Chord V
        for _ in 0..4 {
            track.note(Eigth, Fith, 1);
            track.note(Eigth, Second, 2);
        }

        // Chord I
        for _ in 0..4 {
            track.note(Eigth, First, 2);
            track.note(Eigth, Fith, 2);
        }
    }

    return track;
}

fn track_bass_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.15);

    for _ in 0..2 {
        // Chord V
        track.note(Whole, Fith, 0);

        // Chord I
        track.note(Whole, First, 1);
    }

    return track;
}

fn track_melody_repeated(instrument: Box<dyn Instrument>, repeat: bool) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, Sixth, 3).dotted();
    track.note(Eigth, Seventh, 3);
    track.note(Quarter, First, 4);
    track.note(Eigth, Seventh, 3);
    track.note(Eigth, Sixth, 3);

    track.note(Quarter, Fith, 3).dotted();
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fith, 3);
    track.note(Eigth, Fourth, 3);
    track.note(Eigth, Third, 3);

    track.note(Quarter, Second, 3).dotted();
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fourth, 3);
    track.note(Quarter, Fith, 3);

    if repeat {
        apply_melody_end_part(&mut track);
    }
    else {
        apply_melody_end_full(&mut track);
    }

    return track;
}

fn track_chords_repeated(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.2);

    // Chord IV
    for _ in 0..4 {
        track.note(Eigth, Fourth, 2);
        track.note(Eigth, First, 3);
    }

    // Chord III
    for _ in 0..4 {
        track.note(Eigth, Third, 2);
        track.note(Eigth, Seventh, 2);
    }

    // Chord V
    for _ in 0..4 {
        track.note(Eigth, Fith, 1);
        track.note(Eigth, Second, 2);
    }

    // Chord I
    for _ in 0..4 {
        track.note(Eigth, First, 2);
        track.note(Eigth, Fith, 2);
    }

    return track;
}

fn track_bass_repeated(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.15);

    // Chord IV
    track.note(Whole, Fourth, 1);

    // Chord III
    track.note(Whole, Third, 1);

    // Chord V
    track.note(Whole, Fith, 0);

    // Chord I
    track.note(Whole, First, 1);

    return track;
}

fn apply_melody_end_full(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;

    track.note(Quarter, Third, 3);
    track.note(Quarter, First, 3);
    track.note(Half, First, 3);
}

fn apply_melody_end_part(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;
    
    track.note(Quarter, Third, 3);
    track.note(Quarter, First, 3);
    track.note(Quarter, First, 3);
    track.note(Eigth, Fourth, 3);
    track.note(Eigth, Fith, 3);
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
