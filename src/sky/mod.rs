use crate::{color::Color, geometry::Vec3};

mod day;
mod uniform;

pub use day::*;
pub use uniform::*;

pub trait Sky: Sync {
    fn at(&self, unit_dir: Vec3<f64>) -> Color;
}
