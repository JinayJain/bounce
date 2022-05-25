use std::fmt::Display;

use crate::{geometry::Vec3, impl_math};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(Vec3<f32>);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f32 {
        self.0.x()
    }

    pub fn g(&self) -> f32 {
        self.0.y()
    }

    pub fn b(&self) -> f32 {
        self.0.z()
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Color(self.0 * other)
    }
}

impl Mul<Color> for f32 {
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

        let r = (r * 255.99) as u8;
        let g = (g * 255.99) as u8;
        let b = (b * 255.99) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

impl_math!(Color);
