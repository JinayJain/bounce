use crate::{geo::Vec3, impl_math};
use core::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3<f64>);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }
}

impl_math!(Color);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.0.x() * 255.99) as u8;
        let g = (self.0.y() * 255.99) as u8;
        let b = (self.0.z() * 255.99) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
