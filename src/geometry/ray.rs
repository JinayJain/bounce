use super::{Point, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point<f32>,
    direction: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Point<f32>, direction: Vec3<f32>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(self) -> Point<f32> {
        self.origin
    }

    pub fn direction(self) -> Vec3<f32> {
        self.direction
    }

    pub fn at(self, t: f32) -> Point<f32> {
        self.origin + (self.direction * t).into()
    }
}
