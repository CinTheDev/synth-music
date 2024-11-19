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
        self.points.push(point);
        self
    }
}

impl Curve for LinearCurve {
    fn get(x: f32) -> f32 {
        return 0.0;
    }
}
