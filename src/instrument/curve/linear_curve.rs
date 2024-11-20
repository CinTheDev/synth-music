use super::{Curve, ScaleType};

pub struct LinearCurve {
    points: Vec<(f32, f32)>,
    horizontal_scale: ScaleType,
    vertical_scale: ScaleType,
}

impl LinearCurve {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            horizontal_scale: ScaleType::Linear,
            vertical_scale: ScaleType::Linear,
        }
    }

    pub fn add_point(mut self, x: f32, y: f32) -> Result<Self, &'static str> {
        let last_point = self.points.last().unwrap_or(&(std::f32::NEG_INFINITY, 0.0));

        if x < last_point.0 {
            return Err("Cannot add point left to other points.");
        }

        self.points.push((x, y));
        Ok(self)
    }

    pub fn set_horizontal_scale(mut self, scale_type: ScaleType) -> Self {
        self.horizontal_scale = scale_type;
        self
    }

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

    fn into_closure(self) -> impl Fn(f32) -> f32 {
        move |x| {
            self.get(x)
        }
    }
}
