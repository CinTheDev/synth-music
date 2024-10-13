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
    });

    assert_eq!(one_four, TimeSignature {
        measure_length: QUARTER,
        beat_length: QUARTER,
        beat_intensities: vec![1.0; 1],
    });

    assert_eq!(three_eight, TimeSignature {
        measure_length: EIGTH * 3,
        beat_length: EIGTH,
        beat_intensities: vec![1.0; 3],
    });

    assert_eq!(six_eight, TimeSignature {
        measure_length: EIGTH * 6,
        beat_length: EIGTH,
        beat_intensities: vec![1.0; 6],
    });

    assert_eq!(five_four, TimeSignature {
        measure_length: WHOLE + QUARTER,
        beat_length: QUARTER,
        beat_intensities: vec![1.0; 5],
    });

    assert_eq!(thirtytwo_one, TimeSignature {
        measure_length: WHOLE * 32,
        beat_length: WHOLE,
        beat_intensities: vec![1.0; 32],
    });
}
