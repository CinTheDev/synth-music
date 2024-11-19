pub mod linear_curve;

pub trait Curve {
    fn get(x: f32) -> f32;
}
