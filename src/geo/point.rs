use crate::impl_math;

use super::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Point(pub Vec3<f64>);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Vec3::new(x, y, z))
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }
}

impl_math!(Point);
