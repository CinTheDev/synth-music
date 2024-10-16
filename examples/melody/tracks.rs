use synth_music::prelude::*;
use tet12::*;
use note::length::*;
use crate::instruments::drumset::DrumsetAction;

// BEGIN PART

pub fn melody_begin<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.note(QUARTER, fifth(3));
    track.note(EIGTH, second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(EIGTH, third(3));
    track.note(EIGTH, second(3));

    track.note(QUARTER, first(3));
    track.note(EIGTH, first(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fifth(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, third(3));

    track.note(QUARTER.dot(), second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(QUARTER, fifth(3));

    track.note(QUARTER, third(3));
    track.note(QUARTER, first(3));
    track.note(QUARTER, first(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, fifth(3));

    return track;
}

pub fn chords_begin<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.set_intensity(0.2);

    for _ in 0..2 {
        apply_chord_fifth(&mut track);
        apply_chord_first(&mut track);
    }

    return track;
}

pub fn bass_begin<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.set_intensity(0.15);

    for _ in 0..2 {
        // Chord V
        track.note(WHOLE, fifth(0));

        // Chord I
        track.note(WHOLE, first(1));
    }

    return track;
}

// REPEATED PART

pub fn melody_repeated<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T, repeat: bool) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.note(QUARTER.dot(), sixth(3));
    track.note(EIGTH, seventh(3));
    track.note(QUARTER, first(4));
    track.note(EIGTH, seventh(3));
    track.note(EIGTH, sixth(3));

    track.note(QUARTER.dot(), fifth(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fifth(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, third(3));

    track.note(QUARTER.dot(), second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(QUARTER, fifth(3));

    if repeat {
        track.note(QUARTER, third(3));
        track.note(QUARTER, first(3));
        track.note(QUARTER, first(3));
        track.note(EIGTH, fourth(3));
        track.note(EIGTH, fifth(3));
    }
    else {
        track.note(QUARTER, third(3));
        track.note(QUARTER, first(3));
        track.note(HALF, first(3));
    }

    return track;
}

pub fn chords_repeated<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.set_intensity(0.2);

    apply_chord_fourth(&mut track);
    apply_chord_third(&mut track);
    apply_chord_fifth(&mut track);
    apply_chord_first(&mut track);

    return track;
}

pub fn bass_repeated<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.set_intensity(0.15);

    // Chord IV
    track.note(WHOLE, fourth(1));

    // Chord III
    track.note(WHOLE, third(1));

    // Chord V
    track.note(WHOLE, fifth(0));

    // Chord I
    track.note(WHOLE, first(1));

    return track;
}

// B SECTION (repeated)

pub fn melody_b_section<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T, repeat: bool) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    sequential_notes!(
        track, HALF,
        fifth(3),
        third(3),
        fourth(3),
        second(3)
    );

    if repeat {
        track.note(HALF, third(3));
        track.note(HALF, first(3));

        track.note(HALF, seventh(2).sharp());
        track.note(HALF, second(3));
    }
    else {
        track.note(QUARTER, third(3));
        track.note(QUARTER, fifth(3));
        track.note(HALF, first(4));

        track.note(WHOLE, seventh(3).sharp());
    }

    return track;
}

pub fn chords_b_section<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(0.4);

    for _ in 0..2 {
        apply_chord_first(&mut track);
        apply_chord_fifth(&mut track);
    }

    return track;
}

pub fn bass_b_section<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(0.2);

    for _ in 0..2 {
        track.note(WHOLE, first(1));
        track.note(WHOLE, fifth(0));
    }

    return track;
}

// Chord functions

fn apply_chord_first<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(EIGTH, first(2));
        notes!(
            track, EIGTH,
            first(2),
            third(2),
            fifth(2)
        );
    }
}

fn apply_chord_third<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(EIGTH, third(2));
        notes!(
            track, EIGTH,
            third(2),
            fifth(2),
            seventh(2)
        );
    }
}

fn apply_chord_fourth<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(EIGTH, fourth(2));
        notes!(
            track, EIGTH,
            fourth(2),
            sixth(2),
            first(3)
        );
    }
}

fn apply_chord_fifth<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(EIGTH, fifth(1));
        notes!(
            track, EIGTH,
            fifth(1),
            seventh(1).sharp(),
            second(2)
        );
    }
}

// DRUMSET

pub fn drumset_4<T>(instrument: T, measures: usize) -> UnboundTrack<DrumsetAction, T>
where 
    T: Instrument<ConcreteValue = DrumsetAction>
{
    use DrumsetAction::*;
    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(0.3);

    for _ in 0..measures {
        sequential_notes!(track, QUARTER,
            Bass, Bass, Bass, Bass
        );
    }

    return track;
}
