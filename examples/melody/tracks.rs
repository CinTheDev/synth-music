use synth_music::prelude::*;
use tet12::*;
use note::Length::*;
use crate::instruments::drumset::DrumsetAction;

// BEGIN PART

pub fn melody_begin<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.note(Quarter, fifth(3));
    track.note(Eigth, second(3));
    track.note(Eigth, third(3));
    track.note(Quarter, fourth(3));
    track.note(Eigth, third(3));
    track.note(Eigth, second(3));

    track.note(Quarter, first(3));
    track.note(Eigth, first(3));
    track.note(Eigth, third(3));
    track.note(Quarter, fifth(3));
    track.note(Eigth, fourth(3));
    track.note(Eigth, third(3));

    track.note(Quarter, second(3)).dotted();
    track.note(Eigth, third(3));
    track.note(Quarter, fourth(3));
    track.note(Quarter, fifth(3));

    track.note(Quarter, third(3));
    track.note(Quarter, first(3));
    track.note(Quarter, first(3));
    track.note(Eigth, fourth(3));
    track.note(Eigth, fifth(3));

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
        track.note(Whole, fifth(0));

        // Chord I
        track.note(Whole, first(1));
    }

    return track;
}

// REPEATED PART

pub fn melody_repeated<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T, repeat: bool) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    track.note(Quarter, sixth(3)).dotted();
    track.note(Eigth, seventh(3));
    track.note(Quarter, first(4));
    track.note(Eigth, seventh(3));
    track.note(Eigth, sixth(3));

    track.note(Quarter, fifth(3)).dotted();
    track.note(Eigth, third(3));
    track.note(Quarter, fifth(3));
    track.note(Eigth, fourth(3));
    track.note(Eigth, third(3));

    track.note(Quarter, second(3)).dotted();
    track.note(Eigth, third(3));
    track.note(Quarter, fourth(3));
    track.note(Quarter, fifth(3));

    if repeat {
        track.note(Quarter, third(3));
        track.note(Quarter, first(3));
        track.note(Quarter, first(3));
        track.note(Eigth, fourth(3));
        track.note(Eigth, fifth(3));
    }
    else {
        track.note(Quarter, third(3));
        track.note(Quarter, first(3));
        track.note(Half, first(3));
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
    track.note(Whole, fourth(1));

    // Chord III
    track.note(Whole, third(1));

    // Chord V
    track.note(Whole, fifth(0));

    // Chord I
    track.note(Whole, first(1));

    return track;
}

// B SECTION (repeated)

pub fn melody_b_section<T: Instrument<ConcreteValue = TET12ConcreteTone>>(instrument: T, repeat: bool) -> UnboundTrack<TET12ScaledTone, T> {
    let mut track = UnboundTrack::new(instrument);

    sequential_notes!(
        track, Half,
        fifth(3),
        third(3),
        fourth(3),
        second(3)
    );

    if repeat {
        track.note(Half, third(3));
        track.note(Half, first(3));

        track.note(Half, seventh(2).sharp());
        track.note(Half, second(3));
    }
    else {
        track.note(Quarter, third(3));
        track.note(Quarter, fifth(3));
        track.note(Half, first(4));

        track.note(Whole, seventh(3).sharp());
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
        track.note(Whole, first(1));
        track.note(Whole, fifth(0));
    }

    return track;
}

// Chord functions

fn apply_chord_first<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(Eigth, first(2));
        notes!(
            track, Eigth,
            first(2),
            third(2),
            fifth(2)
        );
    }
}

fn apply_chord_third<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(Eigth, third(2));
        notes!(
            track, Eigth,
            third(2),
            fifth(2),
            seventh(2)
        );
    }
}

fn apply_chord_fourth<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(Eigth, fourth(2));
        notes!(
            track, Eigth,
            fourth(2),
            sixth(2),
            first(3)
        );
    }
}

fn apply_chord_fifth<T: Instrument<ConcreteValue = TET12ConcreteTone>>(track: &mut UnboundTrack<TET12ScaledTone, T>) {
    for _ in 0..4 {
        track.note(Eigth, fifth(1));
        notes!(
            track, Eigth,
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
        sequential_notes!(track, Quarter,
            Bass, Bass, Bass, Bass
        );
    }

    return track;
}
