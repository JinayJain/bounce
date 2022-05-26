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

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color(self.0 * other)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        other * self
    }
}

fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x > max {
        return max;
    }

    if x < min {
        return min;
    }

    x
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = clamp(self.r(), 0.0, 0.999);
        let g = clamp(self.g(), 0.0, 0.999);
        let b = clamp(self.b(), 0.0, 0.999);

        let r = (r * 256.0) as u8;
        let g = (g * 256.0) as u8;
        let b = (b * 256.0) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

impl_math!(Color);
