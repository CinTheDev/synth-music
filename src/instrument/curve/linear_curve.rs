use super::Curve;

pub struct LinearCurve {
    points: Vec<(f32, f32)>,
}

impl LinearCurve {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: (f32, f32)) -> &mut Self {
        let last_point = self.points.last().unwrap_or(&(std::f32::NEG_INFINITY, 0.0));

        if point.0 < last_point.0 {
            panic!("Cannot add point left to other points.");
        }

        self.points.push(point);
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

        let t = (x - left_point.0) / (right_point.0 - left_point.0);
        return t * (right_point.1 - left_point.1) + left_point.1;
    }
}
