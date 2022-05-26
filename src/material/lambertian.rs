use crate::{
    color::Color,
    geometry::{Ray, Vec3},
    object::HitRecord,
};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_dir = hit.normal + Vec3::random_unit();
        let scattered = if scatter_dir.near_zero() {
            // Prevent cases where the ray bounce is 0, leading to NaN/infinites
            Ray::new(hit.point, hit.normal)
        } else {
            Ray::new(hit.point, scatter_dir)
        };

        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
