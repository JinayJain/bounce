use std::fmt::Display;

use crate::{geometry::Vec3, impl_math};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(Vec3<f64>);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = (self.r() * 255.99) as u8;
        let g = (self.g() * 255.99) as u8;
        let b = (self.b() * 255.99) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

impl_math!(Color);
