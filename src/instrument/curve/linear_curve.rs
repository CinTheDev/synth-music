use super::{Curve, ScaleType};

/// An implementor for `Curve`, the function is a set of connected points that
/// interpolate linearly inbetween each other. The axis scale can be set to
/// other types than linear for e.g. interpolating values across a logarithmic
/// scale.
/// 
/// The leftmost and rightmost points cannot interpolate between two values
/// (as there's only one value specified), so the curve is extended as a
/// constant function from these points. If you e.g. want to get a value that
/// is to the left of all specified points, the value will just be the height
/// of the leftmost point. Visually, this is a horizontal line attached to the
/// left of the leftmost point and to the right of the rightmost point.
/// 
/// ```
/// use synth_music::prelude::*;
/// 
/// # let settings = CompositionSettings {
/// #     sample_rate: 48000,
/// # };
/// # let mut buffer = SoundBuffer::new(settings);
/// #
/// let curve = LinearCurve::new()
///     .add_point(0.1, 0.0)
///     .add_point(0.3, 0.5)
///     .add_point(0.4, 1.0)
///     .add_point(0.5, 0.0)
///     .set_vertical_scale(ScaleType::Logarithmic);
/// 
/// eq::filter_fft(&mut buffer, curve.into_closure());
/// ```
/// 
/// If you place a new point inbetween two already existing points, the new
/// point will then be next to the old points; the old points are no longer
/// next to each other.
/// 
/// This way it's impossible to have the function "go backwards". It also means
/// that the order in which points are added does not matter.
/// 
/// ```
/// # use synth_music::prelude::*;
/// #
/// // This is the same as...
/// let curve = LinearCurve::new()
///     .add_point(0.2, 0.0).unwrap()
///     .add_point(0.4, 0.8).unwrap()
///     .add_point(0.5, 1.0).unwrap()
///     .add_point(0.8, 0.0).unwrap();
/// 
/// // ...this curve
/// let curve = LinearCurve::new()
///     .add_point(0.2, 0.0).unwrap()
///     .add_point(0.5, 1.0).unwrap() // this and the element below are swapped.
///     .add_point(0.4, 0.8).unwrap()
///     .add_point(0.8, 0.0).unwrap();
/// ```
/// 
/// It is not checked if new points have precisely the same x value as other
/// points, so this will result in undefined behaviour:
/// 
/// ```
/// # use synth_music::prelude::*;
/// #
/// // This won't panic, but we do not know if the value at `x = 0` will get us
/// // `0` or `1`. Furthermore, it's not defined which value is used for
/// // interpolation.
/// let curve = LinearCurve::new()
///     .add_point(0.0, 0.0)
///     .add_point(0.0, 1.0)
///     .add_point(1.0, 0.5);
/// ```
pub struct LinearCurve {
    points: Vec<(f32, f32)>,
    horizontal_scale: ScaleType,
    vertical_scale: ScaleType,
}

impl LinearCurve {
    /// Create a new empty `LinearCurve` with the axes scale type set to
    /// `Linear`.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            horizontal_scale: ScaleType::Linear,
            vertical_scale: ScaleType::Linear,
        }
    }

    /// Add a point to the function.
    pub fn add_point(mut self, x: f32, y: f32) -> Self {
        for i in 0..self.points.len() {
            let right_point = self.points[i];

            if x < right_point.0 {
                self.points.insert(i, (x, y));
                return self
            }
        }

        // If the new point is to the right of all other points
        self.points.push((x, y));
        return self;
    }

    /// Set the scale type of the horizontal x-axis.
    pub fn set_horizontal_scale(mut self, scale_type: ScaleType) -> Self {
        self.horizontal_scale = scale_type;
        self
    }

    /// Set the scale type of the vertical y-axis.
    pub fn set_vertical_scale(mut self, scale_type: ScaleType) -> Self {
        self.vertical_scale = scale_type;
        self
    }
}

impl Curve for LinearCurve {
    fn get(&self, x: f32) -> f32 {
        let mut left_point = (std::f32::NAN, std::f32::NAN);
        let mut right_point = (std::f32::NAN, std::f32::NAN);

        for point in &self.points {
            if point.0 > x {
                right_point = *point;
                break;
            }

            left_point = *point;
        }

        if left_point.0.is_nan() {
            return right_point.1;
        }
        if right_point.0.is_nan() {
            return left_point.1;
        }

        let t = self.horizontal_scale.interpolate_inverse(left_point.0, right_point.0, x);
        return self.vertical_scale.interpolate(left_point.1, right_point.1, t);
    }
}
