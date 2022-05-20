use super::{Double, Point, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point<Double>,
    direction: Vec3<Double>,
}

impl Ray {
    pub fn new(origin: Point<Double>, direction: Vec3<Double>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(self) -> Point<Double> {
        self.origin
    }

    pub fn direction(self) -> Vec3<Double> {
        self.direction
    }

    pub fn at(self, t: Double) -> Point<Double> {
        self.origin + (self.direction * t).into()
    }
}
