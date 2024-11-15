#![cfg(test)]

use super::*;
use crate::prelude::length::*;

#[test]
fn test_construction() {
    let four_four = TimeSignature::new(4, 4);
    let one_four = TimeSignature::new(1, 4);
    let three_eight = TimeSignature::new(3, 8);
    let six_eight = TimeSignature::new(6, 8);
    let five_four = TimeSignature::new(5, 4);
    let thirtytwo_one = TimeSignature::new(32, 1);

    assert_eq!(four_four, TimeSignature {
        measure_length: WHOLE,
        beat_length: QUARTER,
        beat_intensities: vec![1.0; 4],
        offbeat_intensity: 1.0,
    });

    assert_eq!(one_four, TimeSignature {
        measure_length: QUARTER,
        beat_length: QUARTER,
        beat_intensities: vec![1.0; 1],
        offbeat_intensity: 1.0,
    });

    assert_eq!(three_eight, TimeSignature {
        measure_length: EIGTH * 3,
        beat_length: EIGTH,
        beat_intensities: vec![1.0; 3],
        offbeat_intensity: 1.0,
    });

    assert_eq!(six_eight, TimeSignature {
        measure_length: EIGTH * 6,
        beat_length: EIGTH,
        beat_intensities: vec![1.0; 6],
        offbeat_intensity: 1.0,
    });

    assert_eq!(five_four, TimeSignature {
        measure_length: WHOLE + QUARTER,
        beat_length: QUARTER,
        beat_intensities: vec![1.0; 5],
        offbeat_intensity: 1.0,
    });

    assert_eq!(thirtytwo_one, TimeSignature {
        measure_length: WHOLE * 32,
        beat_length: WHOLE,
        beat_intensities: vec![1.0; 32],
        offbeat_intensity: 1.0,
    });
}

#[test]
#[should_panic]
fn test_wrong_construction_1() {
    TimeSignature::new(4, 5);
}

#[test]
#[should_panic]
fn test_wrong_construction_2() {
    TimeSignature::new(4, 0);
}

#[test]
#[should_panic]
fn test_wrong_construction_3() {
    TimeSignature::new(0, 4);
}

#[test]
fn test_measure_saturation() {
    let four_four = TimeSignature::new(4, 4);
    let two_four = TimeSignature::new(2, 4);

    let six_eight = TimeSignature::new(6, 8);
    let five_four = TimeSignature::new(5, 4);
    let thirtytwo_one = TimeSignature::new(32, 1);
    let one_sixteen = TimeSignature::new(1, 16);

    assert!(four_four.is_measure_saturated(WHOLE));
    assert!(two_four.is_measure_saturated(HALF));
    assert!(six_eight.is_measure_saturated(QUARTER * 3));
    assert!(five_four.is_measure_saturated(WHOLE + QUARTER));
    assert!(thirtytwo_one.is_measure_saturated(WHOLE * 32));
    assert!(one_sixteen.is_measure_saturated(SIXTEENTH));

    assert!(! four_four.is_measure_saturated(WHOLE + SIXTEENTH));
    assert!(! six_eight.is_measure_saturated(QUARTER * 3 - EIGTH));
}
