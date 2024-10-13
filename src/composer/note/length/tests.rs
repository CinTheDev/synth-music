#![cfg(test)]

use super::*;

#[test]
fn test_construction_subdivisions() {
    let whole = Length::from_subdivisions(0);
    let half = Length::from_subdivisions(1);
    let quarter = Length::from_subdivisions(2);
    let smallest = Length::from_subdivisions(16);

    assert_eq!(whole,    Length::from_ticks(2_u32.pow(16)));
    assert_eq!(half,     Length::from_ticks(2_u32.pow(15)));
    assert_eq!(quarter,  Length::from_ticks(2_u32.pow(14)));
    assert_eq!(smallest, Length::from_ticks(1));
}

#[test]
fn test_dot() {
    let quarter_dot = QUARTER.dot();
    let sixteenth_dot = SIXTEENTH.dot();
    
    let whole_dot = WHOLE.dot();
    let whole4_dot = (WHOLE * 4).dot();

    assert_eq!(quarter_dot, QUARTER + EIGTH);
    assert_eq!(sixteenth_dot, SIXTEENTH + Length::from_subdivisions(5));
    assert_eq!(whole_dot, WHOLE + HALF);
    assert_eq!(whole4_dot, WHOLE * 6);
}

#[test]
fn test_multidot() {
    let quarter_dot = QUARTER.multi_dot(1);
    let quarter_2dot = QUARTER.multi_dot(2);

    let whole_3dot = WHOLE.multi_dot(3);
    let whole4_4dot = (WHOLE * 4).multi_dot(4);

    assert_eq!(quarter_dot, QUARTER + EIGTH);
    assert_eq!(quarter_2dot, QUARTER + EIGTH + SIXTEENTH);
    assert_eq!(whole_3dot, WHOLE + HALF + QUARTER);
    assert_eq!(whole4_4dot, WHOLE*4 + WHOLE*2 + WHOLE + HALF);
}

#[test]
fn test_tofloat() {
    let whole = WHOLE.to_float();
    let quarter = QUARTER.to_float();

    let quarter_dot = QUARTER.dot().to_float();
    let half_2dot = HALF.multi_dot(2).to_float();

    let quarter_triole = QUARTER.triole().to_float();
    let half_pentole = HALF.ntole(5).to_float();

    let whole2_2dot = (WHOLE * 2).multi_dot(2).to_float();

    let epsilon = 0.001;

    assert_eq_f32(whole, 1.0, epsilon);
    assert_eq_f32(quarter, 0.25, epsilon);
    assert_eq_f32(quarter_dot, 0.25 + 0.125, epsilon);
    assert_eq_f32(half_2dot, 0.5 + 0.25 + 0.125, epsilon);
    assert_eq_f32(quarter_triole, 0.5 / 3.0, epsilon);
    assert_eq_f32(half_pentole, 1.0 / 5.0, epsilon);
    assert_eq_f32(whole2_2dot, 2.0 + 1.0 + 0.5, epsilon);
}

// Utility functions

fn assert_eq_f32(a: f32, b: f32, epsilon: f32) {
    let delta = (a - b).abs();
    if delta > epsilon {
        panic!("assertion failed: {} != {}", a, b);
    }
}
