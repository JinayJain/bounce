use crate::{color::Color, geometry::Vec3};

use super::Sky;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sky for Day {
    fn at(&self, unit_dir: Vec3<f64>) -> Color {
        let t = 0.5 * (unit_dir.y() + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
