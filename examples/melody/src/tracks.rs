use unnamed_music::melody::prelude::*;
use unnamed_music::{notes, sequential_notes};

// BEGIN PART

pub fn melody_begin(instrument: Box<dyn Instrument>) -> Track {
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

    track.note(Quarter, Third, 3);
    track.note(Quarter, First, 3);
    track.note(Quarter, First, 3);
    track.note(Eigth, Fourth, 3);
    track.note(Eigth, Fith, 3);

    return track;
}

pub fn chords_begin(instrument: Box<dyn Instrument>) -> Track {
    let mut track = Track::new(instrument);

    track.set_intensity(0.2);

    for _ in 0..2 {
        apply_chord_fifth(&mut track);
        apply_chord_first(&mut track);
    }

    return track;
}

pub fn bass_begin(instrument: Box<dyn Instrument>) -> Track {
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

// REPEATED PART

pub fn melody_repeated(instrument: Box<dyn Instrument>, repeat: bool) -> Track {
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
        track.note(Quarter, Third, 3);
        track.note(Quarter, First, 3);
        track.note(Quarter, First, 3);
        track.note(Eigth, Fourth, 3);
        track.note(Eigth, Fith, 3);
    }
    else {
        track.note(Quarter, Third, 3);
        track.note(Quarter, First, 3);
        track.note(Half, First, 3);
    }

    return track;
}

pub fn chords_repeated(instrument: Box<dyn Instrument>) -> Track {
    let mut track = Track::new(instrument);

    track.set_intensity(0.2);

    apply_chord_fourth(&mut track);
    apply_chord_third(&mut track);
    apply_chord_fifth(&mut track);
    apply_chord_first(&mut track);

    return track;
}

pub fn bass_repeated(instrument: Box<dyn Instrument>) -> Track {
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

// B SECTION (repeated)

pub fn melody_b_section(instrument: Box<dyn Instrument>, repeat: bool) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    sequential_notes!(
        track, Half,
        (Fith, 3),
        (Third, 3),
        (Fourth, 3),
        (Second, 3)
    );

    if repeat {
        track.note(Half, Third, 3);
        track.note(Half, First, 3);

        track.note(Half, Seventh, 2).sharp();
        track.note(Half, Second, 3);
    }
    else {
        track.note(Quarter, Third, 3);
        track.note(Quarter, Fith, 3);
        track.note(Half, First, 4);

        track.note(Whole, Seventh, 3).sharp();
    }

    return track;
}

pub fn chords_b_section(instrument: Box<dyn Instrument>) -> Track {
    let mut track = Track::new(instrument);
    track.set_intensity(0.4);

    for _ in 0..2 {
        apply_chord_first(&mut track);
        apply_chord_fifth(&mut track);
    }

    return track;
}

pub fn bass_b_section(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;

    let mut track = Track::new(instrument);
    track.set_intensity(0.2);

    for _ in 0..2 {
        track.note(Whole, First, 1);
        track.note(Whole, Fith, 0);
    }

    return track;
}

// Chord functions

fn apply_chord_first(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;

    for _ in 0..4 {
        track.note(Eigth, First, 2);
        notes!(
            track, Eigth,
            (First, 2),
            (Third, 2),
            (Fith, 2)
        );
    }
}

fn apply_chord_third(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;

    for _ in 0..4 {
        track.note(Eigth, Third, 2);
        notes!(
            track, Eigth,
            (Third, 2),
            (Fith, 2),
            (Seventh, 2)
        );
    }
}

fn apply_chord_fourth(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;

    for _ in 0..4 {
        track.note(Eigth, Fourth, 2);
        notes!(
            track, Eigth,
            (Fourth, 2),
            (Sixth, 2),
            (First, 3)
        );
    }
}

fn apply_chord_fifth(track: &mut Track) {
    use note::Tone::*;
    use note::Length::*;

    for _ in 0..4 {
        track.note(Eigth, Fith, 1);
        notes!(
            track, Eigth,
            (Fith, 1),
            //(Seventh, 1),
            (Second, 2)
        );
    }
}
