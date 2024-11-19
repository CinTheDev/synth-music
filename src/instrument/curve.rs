pub mod linear_curve;

pub trait Curve {
    fn get(&self, x: f32) -> f32;
}

pub enum ScaleType {
    Linear,
    Logarithmic,
}
