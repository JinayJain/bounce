use crate::{color::Color, geometry::Vec3};

use super::Sky;

pub struct Uniform {
    color: Color,
}

impl Uniform {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Sky for Uniform {
    fn at(&self, _unit_dir: Vec3<f64>) -> Color {
        return self.color;
    }
}
