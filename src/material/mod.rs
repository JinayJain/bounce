use crate::{color::Color, geometry::Ray, object::HitRecord};

mod lambertian;
mod metal;

pub use lambertian::*;
pub use metal::*;

pub trait Material {
    fn scatter(&self, r: Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}
