use super::{point::Point, Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        let Point(origin) = self.origin;

        Point(origin + self.direction * t)
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }
}
