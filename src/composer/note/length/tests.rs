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
