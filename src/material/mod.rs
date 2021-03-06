use crate::{color::Color, geometry::Ray, object::VisibleHit};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

pub trait Material: Sync + Send {
    fn scatter(&self, r: Ray, hit: &VisibleHit) -> Option<(Ray, Color)>;
}
