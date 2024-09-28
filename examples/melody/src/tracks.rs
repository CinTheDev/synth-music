use unnamed_music::melody::prelude::*;

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

    apply_melody_end_part(&mut track);

    return track;
}

pub fn chords_begin(instrument: Box<dyn Instrument>) -> Track {
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
        apply_melody_end_part(&mut track);
    }
    else {
        apply_melody_end_full(&mut track);
    }

    return track;
}

pub fn chords_repeated(instrument: Box<dyn Instrument>) -> Track {
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
