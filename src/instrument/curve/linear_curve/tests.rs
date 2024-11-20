#![cfg(test)]

use super::*;

#[test]
fn linear_curve() {
    let curve = LinearCurve::new()
        .add_point(0.0, 0.0)
        .add_point(0.5, 1.0)
        .add_point(1.0, 0.5);

    // Check specified points
    assert!(check_curve(&curve, 0.0, 0.0));
    assert!(check_curve(&curve, 0.5, 1.0));
    assert!(check_curve(&curve, 1.0, 0.5));

    // Check points outside range
    assert!(check_curve(&curve, -1.0, 0.0));
    assert!(check_curve(&curve, 3.0, 0.5));

    // Check interpolation
    assert!(check_curve(&curve, 0.25, 0.5));
    assert!(check_curve(&curve, 0.75, 0.75));

    // Second set of points
    let curve = LinearCurve::new()
        .add_point(1.0, 100.0)
        .add_point(1.5, 100.0)
        .add_point(-19.0, 0.0)
        .add_point(2.0, 90.0);

    // Check specified points
    assert!(check_curve(&curve, -19.0, 0.0));
    assert!(check_curve(&curve, 1.0, 100.0));
    assert!(check_curve(&curve, 1.5, 100.0));
    assert!(check_curve(&curve, 2.0, 90.0));

    // Check points outside range
    assert!(check_curve(&curve, -100.0, 0.0));
    assert!(check_curve(&curve,  100.0, 90.0));

    // Check interpolation
    assert!(check_curve(&curve, -9.0, 50.0));
    assert!(check_curve(&curve, 1.2, 100.0));
    assert!(check_curve(&curve, 1.75, 95.0));
}

fn check_curve(
    curve: &LinearCurve,
    point: f32,
    expected: f32,
) -> bool {
    let result = curve.get(point);
    return eq_f32(result, expected, 0.01);
}

fn eq_f32(a: f32, b: f32, epsilon: f32) -> bool {
    let delta = (a - b).abs();
    delta <= epsilon
}
