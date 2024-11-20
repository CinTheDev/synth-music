pub mod linear_curve;

pub trait Curve {
    fn get(&self, x: f32) -> f32;
    fn into_closure(self) -> impl Fn(f32) -> f32;
}

#[derive(Clone, Copy)]
pub enum ScaleType {
    Linear,
    Logarithmic,
}

impl ScaleType {
    pub fn interpolate(self, a: f32, b: f32, t: f32) -> f32 {
        match self {
            Self::Linear => { t * (b - a) + a },

            Self::Logarithmic => {
                let lerp = t * (b.log10() - a.log10()) + a.log10();
                10_f32.powf(lerp)
            },
        }
    }

    pub fn interpolate_inverse(self, a: f32, b: f32, p: f32) -> f32 {
        match self {
            Self::Linear => { (p - a) / (b - a) },

            Self::Logarithmic => {
                (p.log10() - a.log10()) / (b.log10() - a.log10())
            },
        }
    }
}
