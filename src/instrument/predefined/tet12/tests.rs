#![cfg(test)]
use super::*;

// Tests for TET12ScaledTone

#[test]
fn test_placement() {
    let first_note = first(1);
    let second_note = second(2);
    let third_note = third(3);
    let fourth_note = fourth(4);
    let fifth_note = fifth(5);
    let sixth_note = sixth(6);
    let seventh_note = seventh(7);

    let first_expected = TET12ScaledTone {
        index: 0,
        octave: 1,
        offset: 0,
    };
    let second_expected = TET12ScaledTone {
        index: 1,
        octave: 2,
        offset: 0,
    };
    let third_expected = TET12ScaledTone {
        index: 2,
        octave: 3,
        offset: 0,
    };
    let fourth_expected = TET12ScaledTone {
        index: 3,
        octave: 4,
        offset: 0,
    };
    let fifth_expected = TET12ScaledTone {
        index: 4,
        octave: 5,
        offset: 0,
    };
    let sixth_expected = TET12ScaledTone {
        index: 5,
        octave: 6,
        offset: 0,
    };
    let seventh_expected = TET12ScaledTone {
        index: 6,
        octave: 7,
        offset: 0,
    };

    let suboctave = second(-2);

    let suboctave_expected = TET12ScaledTone {
        index: 1,
        octave: -2,
        offset: 0,
    };

    assert_eq!(first_note, first_expected);
    assert_eq!(second_note, second_expected);
    assert_eq!(third_note, third_expected);
    assert_eq!(fourth_note, fourth_expected);
    assert_eq!(fifth_note, fifth_expected);
    assert_eq!(sixth_note, sixth_expected);
    assert_eq!(seventh_note, seventh_expected);
    assert_eq!(suboctave, suboctave_expected);
}

#[test]
fn test_offset() {
    let note_sharp = first(3).sharp();
    let note_flat = second(1).flat();

    let note_neutral = fifth(2).sharp().flat();

    let note_double_sharp = seventh(5).sharp().sharp();
    let note_double_flat = seventh(5).flat().flat();

    assert_eq!(note_sharp, TET12ScaledTone {
        index: 0,
        octave: 3,
        offset: 1,
    });

    assert_eq!(note_flat, TET12ScaledTone {
        index: 1,
        octave: 1,
        offset: -1,
    });

    assert_eq!(note_neutral, TET12ScaledTone {
        index: 4,
        octave: 2,
        offset: 0,
    });

    assert_eq!(note_double_sharp, TET12ScaledTone {
        index: 6,
        octave: 5,
        offset: 2,
    });

    assert_eq!(note_double_flat, TET12ScaledTone {
        index: 6,
        octave: 5,
        offset: -2,
    });
}
