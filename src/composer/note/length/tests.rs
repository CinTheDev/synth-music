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
