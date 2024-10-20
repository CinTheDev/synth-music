use synth_music::prelude::*;
use tet12::*;
use length::*;
use crate::instruments::DrumsetAction;

// BEGIN PART

fn melody_beat() -> TimeSignature {
    TimeSignature::new(4, 4)
}

fn chords_beat() -> TimeSignature {
    TimeSignature::new(4, 4)
}

fn bass_beat() -> TimeSignature {
    TimeSignature::new(4, 4)
}

fn drums_beat() -> TimeSignature {
    TimeSignature::new(4, 4)
}

pub fn melody_begin<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, melody_beat());
    track.set_intensity(0.5);

    track.note(QUARTER, fifth(3));
    track.note(EIGTH, second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(EIGTH, third(3));
    track.note(EIGTH, second(3));
    track.measure().unwrap();

    track.note(QUARTER, first(3));
    track.note(EIGTH, first(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fifth(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, third(3));
    track.measure().unwrap();

    track.note(QUARTER.dot(), second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(QUARTER, fifth(3));
    track.measure().unwrap();

    track.note(QUARTER, third(3));
    track.note(QUARTER, first(3));
    track.note(QUARTER, first(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, fifth(3));
    track.measure().unwrap();

    return track;
}

pub fn chords_begin<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, chords_beat());

    track.set_intensity(0.3);

    for _ in 0..2 {
        apply_chord_fifth(&mut track);
        apply_chord_first(&mut track);
    }

    return track;
}

pub fn bass_begin<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, bass_beat());

    track.set_intensity(0.25);

    for _ in 0..2 {
        // Chord V
        track.note(WHOLE, fifth(1));
        track.measure().unwrap();

        // Chord I
        track.note(WHOLE, first(1));
        track.measure().unwrap();
    }

    return track;
}

// REPEATED PART

pub fn melody_repeated<T>(instrument: T, repeat: bool) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, melody_beat());
    track.set_intensity(0.5);

    track.note(QUARTER.dot(), sixth(3));
    track.note(EIGTH, seventh(3));
    track.note(QUARTER, first(4));
    track.note(EIGTH, seventh(3));
    track.note(EIGTH, sixth(3));
    track.measure().unwrap();

    track.note(QUARTER.dot(), fifth(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fifth(3));
    track.note(EIGTH, fourth(3));
    track.note(EIGTH, third(3));
    track.measure().unwrap();

    track.note(QUARTER.dot(), second(3));
    track.note(EIGTH, third(3));
    track.note(QUARTER, fourth(3));
    track.note(QUARTER, fifth(3));
    track.measure().unwrap();

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
    track.measure().unwrap();

    return track;
}

pub fn chords_repeated<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, chords_beat());

    track.set_intensity(0.3);

    apply_chord_fourth(&mut track);
    apply_chord_third(&mut track);
    apply_chord_fifth(&mut track);
    apply_chord_first(&mut track);

    return track;
}

pub fn bass_repeated<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, bass_beat());

    track.set_intensity(0.25);

    // Chord IV
    track.note(WHOLE, fourth(1));
    track.measure().unwrap();

    // Chord III
    track.note(WHOLE, third(1));
    track.measure().unwrap();

    // Chord V
    track.note(WHOLE, fifth(0));
    track.measure().unwrap();

    // Chord I
    track.note(WHOLE, first(1));
    track.measure().unwrap();

    return track;
}

// B SECTION (repeated)

pub fn melody_b_section<T>(instrument: T, repeat: bool) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, melody_beat());
    track.set_intensity(0.1);

    sequential_notes!(
        track, HALF,
        fifth(3),
        third(3),
    );
    track.measure().unwrap();
    sequential_notes!(
        track, HALF,
        fourth(3),
        second(3)
    );
    track.measure().unwrap();

    if repeat {
        track.note(HALF, third(3));
        track.note(HALF, first(3));
        track.measure().unwrap();

        track.note(HALF, seventh(2).sharp());
        track.note(HALF, second(3));
        track.measure().unwrap();
    }
    else {
        track.note(QUARTER, third(3));
        track.note(QUARTER, fifth(3));
        track.note(HALF, first(4));
        track.measure().unwrap();

        track.note(WHOLE, seventh(3).sharp());
        track.measure().unwrap();
    }

    return track;
}

pub fn chords_b_section<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, chords_beat());
    track.set_intensity(0.4);

    for _ in 0..2 {
        apply_chord_first(&mut track);
        apply_chord_fifth(&mut track);
    }

    return track;
}

pub fn bass_b_section<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    let mut track = MeasureTrack::new(instrument, bass_beat());
    track.set_intensity(0.2);

    for _ in 0..2 {
        track.note(WHOLE, first(1));
        track.measure().unwrap();

        track.note(WHOLE, fifth(0));
        track.measure().unwrap();
    }

    return track;
}

// Chord functions

fn apply_chord_first<T>(track: &mut MeasureTrack<TET12ScaledTone, T>)
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    for _ in 0..4 {
        track.note(EIGTH, first(2));
        notes!(
            track, EIGTH,
            first(2),
            third(2),
            fifth(2)
        );
    }
    track.measure().unwrap();
}

fn apply_chord_third<T>(track: &mut MeasureTrack<TET12ScaledTone, T>)
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    for _ in 0..4 {
        track.note(EIGTH, third(2));
        notes!(
            track, EIGTH,
            third(2),
            fifth(2),
            seventh(2)
        );
    }
    track.measure().unwrap();
}

fn apply_chord_fourth<T>(track: &mut MeasureTrack<TET12ScaledTone, T>)
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    for _ in 0..4 {
        track.note(EIGTH, fourth(2));
        notes!(
            track, EIGTH,
            fourth(2),
            sixth(2),
            first(3)
        );
    }
    track.measure().unwrap();
}

fn apply_chord_fifth<T>(track: &mut MeasureTrack<TET12ScaledTone, T>)
where 
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    for _ in 0..4 {
        track.note(EIGTH, fifth(1));
        notes!(
            track, EIGTH,
            seventh(1).sharp(),
            second(2),
            fifth(2),
        );
    }
    track.measure().unwrap();
}

// DRUMSET

pub fn drumset_4<T>(instrument: T, measures: usize) -> MeasureTrack<DrumsetAction, T>
where 
    T: Instrument<ConcreteValue = DrumsetAction>
{
    use DrumsetAction::*;

    let mut track = MeasureTrack::new(instrument, drums_beat());
    track.set_intensity(0.5);

    for _ in 0..measures {
        sequential_notes!(track, QUARTER,
            Bass, Bass, Bass, Bass
        );
        track.measure().unwrap();
    }

    return track;
}
