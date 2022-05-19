use core::fmt;

use super::Vec3;

pub type Color = Vec3<f64>;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.x * 255.99) as u8;
        let g = (self.y * 255.99) as u8;
        let b = (self.z * 255.99) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
