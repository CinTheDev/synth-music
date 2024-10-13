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

// Tests for TET12ConcreteTone

#[test]
fn test_conversion_c_major() {
    let key = crate::prelude::music_key::C_MAJOR;

    let c2 = first(2).to_concrete_value(key);
    let d2 = second(2).to_concrete_value(key);
    let e3 = third(3).to_concrete_value(key);
    let f3 = fourth(3).to_concrete_value(key);
    let g4 = fifth(4).to_concrete_value(key);
    let a4 = sixth(4).to_concrete_value(key);
    let b5 = seventh(5).to_concrete_value(key);

    let c2_sharp = first(2).sharp().to_concrete_value(key);
    let d2_flat = second(2).flat().to_concrete_value(key);
    let e3_sharp = third(3).sharp().to_concrete_value(key);
    let f3_flat = fourth(3).flat().to_concrete_value(key);
    let g4_sharp = fifth(4).sharp().to_concrete_value(key);
    let a4_flat = sixth(4).flat().to_concrete_value(key);
    let b5_sharp = seventh(5).sharp().to_concrete_value(key);

    assert_eq!(c2, TET12ConcreteTone(-9 - 24));
    assert_eq!(d2, TET12ConcreteTone(-7 - 24));
    assert_eq!(e3, TET12ConcreteTone(-5 - 12));
    assert_eq!(f3, TET12ConcreteTone(-4 - 12));
    assert_eq!(g4, TET12ConcreteTone(-2     ));
    assert_eq!(a4, TET12ConcreteTone( 0     ));
    assert_eq!(b5, TET12ConcreteTone( 2 + 12));

    assert_eq!(c2_sharp, TET12ConcreteTone(-9 - 24 + 1));
    assert_eq!(d2_flat,  TET12ConcreteTone(-7 - 24 - 1));
    assert_eq!(e3_sharp, TET12ConcreteTone(-5 - 12 + 1));
    assert_eq!(f3_flat,  TET12ConcreteTone(-4 - 12 - 1));
    assert_eq!(g4_sharp, TET12ConcreteTone(-2      + 1));
    assert_eq!(a4_flat,  TET12ConcreteTone( 0      - 1));
    assert_eq!(b5_sharp, TET12ConcreteTone( 2 + 12 + 1));
}

#[test]
fn test_conversion_f_minor() {
    let key = crate::prelude::music_key::F_MINOR;

    let f2 = first(2).to_concrete_value(key);
    let g2 = second(2).to_concrete_value(key);
    let a3_flat = third(3).to_concrete_value(key);
    let b3_flat = fourth(3).to_concrete_value(key);
    let c4 = fifth(4).to_concrete_value(key);
    let d4_flat = sixth(4).to_concrete_value(key);
    let e5_flat = seventh(5).to_concrete_value(key);

    assert_eq!(f2,      TET12ConcreteTone(-4 - 24    ));
    assert_eq!(g2,      TET12ConcreteTone(-2 - 24    ));
    assert_eq!(a3_flat, TET12ConcreteTone( 0 - 12 - 1));
    assert_eq!(b3_flat, TET12ConcreteTone( 2 - 12 - 1));
    assert_eq!(c4,      TET12ConcreteTone(-9         ));
    assert_eq!(d4_flat, TET12ConcreteTone(-7      - 1));
    assert_eq!(e5_flat, TET12ConcreteTone(-5 + 12 - 1));
}

#[test]
fn test_conversion_a_major() {
    let key = crate::prelude::music_key::A_MAJOR;

    let a4 = first(4).to_concrete_value(key);
    let b4 = second(4).to_concrete_value(key);
    let c5 = third(5).to_concrete_value(key);
    let d5 = fourth(5).to_concrete_value(key);
    let e5 = fifth(5).to_concrete_value(key);
    let f5 = sixth(5).to_concrete_value(key);
    let g5 = seventh(5).to_concrete_value(key);

    assert_eq!(a4, TET12ConcreteTone( 0));
    assert_eq!(b4, TET12ConcreteTone( 2));
    assert_eq!(c5, TET12ConcreteTone( 3));
    assert_eq!(d5, TET12ConcreteTone( 5));
    assert_eq!(e5, TET12ConcreteTone( 7));
    assert_eq!(f5, TET12ConcreteTone( 8));
    assert_eq!(g5, TET12ConcreteTone(10));
}

#[test]
fn test_frequency_conversion() {
    let key = crate::prelude::music_key::A_MINOR;

    let a4 = first(4).to_concrete_value(key).to_frequency();
    let b4 = second(4).to_concrete_value(key).to_frequency();
    let c5 = third(5).to_concrete_value(key).to_frequency();
    let d5 = fourth(5).to_concrete_value(key).to_frequency();
    let e5 = fifth(5).to_concrete_value(key).to_frequency();
    let f5 = sixth(5).to_concrete_value(key).to_frequency();
    let g5 = seventh(5).to_concrete_value(key).to_frequency();

    let a4_expected = 440.0 * 2_f32.powf(0.0  / 12.0);
    let b4_expected = 440.0 * 2_f32.powf(2.0  / 12.0);
    let c5_expected = 440.0 * 2_f32.powf(3.0  / 12.0);
    let d5_expected = 440.0 * 2_f32.powf(5.0  / 12.0);
    let e5_expected = 440.0 * 2_f32.powf(7.0  / 12.0);
    let f5_expected = 440.0 * 2_f32.powf(8.0  / 12.0);
    let g5_expected = 440.0 * 2_f32.powf(10.0 / 12.0);

    let epsilon = 0.001;

    assert_eq_f32(a4, a4_expected, epsilon);
    assert_eq_f32(b4, b4_expected, epsilon);
    assert_eq_f32(c5, c5_expected, epsilon);
    assert_eq_f32(d5, d5_expected, epsilon);
    assert_eq_f32(e5, e5_expected, epsilon);
    assert_eq_f32(f5, f5_expected, epsilon);
    assert_eq_f32(g5, g5_expected, epsilon);
}

// Utility functions

fn assert_eq_f32(a: f32, b: f32, epsilon: f32) {
    let delta = (a - b).abs();

    if delta > epsilon {
        panic!("assertion failed: {} != {}", a, b);
    }
}
