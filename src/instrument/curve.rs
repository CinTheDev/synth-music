pub mod linear_curve;

/// A trait to represent a function graph.
pub trait Curve {
    /// Plug x in the function to compute y.
    fn get(&self, x: f32) -> f32;

    /// Return a closure where the input is x and the output is the computed y
    /// value. The default implementation calls `get(x)` and probably does not
    /// need to be implemented manually.
    fn into_closure(&self) -> impl Fn(f32) -> f32 {
        move |x| {
            self.get(x)
        }
    }
}

/// Represent the type of scale used for `Curve`.
/// 
/// E.g. a linear scale will make the Curve axes linear, and logarithmic will
/// make the axes scale logarithmically.
#[derive(Clone, Copy)]
pub enum ScaleType {
    Linear,
    Logarithmic,
}

impl ScaleType {
    /// Interpolate from `a` to `b` using `t`, where `t == 0` will result in `a`
    /// and `t == 1` will result in `b`. How the values in between are scaled
    /// depend on the scale type.
    pub fn interpolate(self, a: f32, b: f32, t: f32) -> f32 {
        match self {
            Self::Linear => { t * (b - a) + a },

            Self::Logarithmic => {
                let lerp = t * (b.log10() - a.log10()) + a.log10();
                10_f32.powf(lerp)
            },
        }
    }

    /// Compute the t value from the range defined by a and b, with the point p
    /// marking the specific point. If `p == a` then t will be zero, and if
    /// `p == b` then t will be one. How the values in between are scaled depend
    /// on the scale type.
    pub fn interpolate_inverse(self, a: f32, b: f32, p: f32) -> f32 {
        match self {
            Self::Linear => { (p - a) / (b - a) },

            Self::Logarithmic => {
                (p.log10() - a.log10()) / (b.log10() - a.log10())
            },
        }
    }
}
