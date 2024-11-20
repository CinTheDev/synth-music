#![cfg(test)]

use super::*;

// Tests for ScaleType

#[test]
fn linear_interpolation() {
    assert!(check_interpolation(
        0.0,
        1.0,
        0.5,
        0.5,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        0.0,
        1.0,
        0.1,
        0.1,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        1.0,
        0.0,
        0.1,
        0.9,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        1.0,
        2.0,
        0.0,
        1.0,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        5.0,
        9.0,
        1.0,
        9.0,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        -5.0,
         5.0,
        0.5,
        0.0,
        ScaleType::Linear,
    ));

    assert!(check_interpolation(
        0.0,
        1.0,
        2.0,
        2.0,
        ScaleType::Linear,
    ));
}

#[test]
fn linear_interpolation_inverse() {
    assert!(check_interpolation_inverse(
        0.0,
        1.0,
        0.5,
        0.5,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        0.0,
        1.0,
        0.1,
        0.1,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        0.0,
        2.0,
        0.5,
        0.25,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        1.0,
        3.0,
        2.0,
        0.5,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        -1.0,
        5.0,
        -1.0,
        0.0,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        13.0,
        0.0,
        0.0,
        1.0,
        ScaleType::Linear,
    ));

    assert!(check_interpolation_inverse(
        0.0,
        0.5,
        1.0,
        2.0,
        ScaleType::Linear,
    ));
}

#[test]
fn logarithmic_interpolation() {
    todo!();
}

#[test]
fn logarithmic_interpolation_inverse() {
    todo!();
}

fn check_interpolation(
    a: f32,
    b: f32,
    t: f32,
    expected: f32,
    scale_type: ScaleType,
) -> bool {
    let result = scale_type.interpolate(a, b, t);
    return eq_f32(result, expected, 0.001);
}

fn check_interpolation_inverse(
    a: f32,
    b: f32,
    p: f32,
    expected: f32,
    scale_type: ScaleType,
) -> bool {
    let result = scale_type.interpolate_inverse(a, b, p);
    return eq_f32(result, expected, 0.001);
}

fn eq_f32(a: f32, b: f32, epsilon: f32) -> bool {
    let delta = (a - b).abs();
    delta <= epsilon
}
