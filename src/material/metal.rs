use crate::{
    color::Color,
    geometry::{Ray, Vec3},
    object::VisibleHit,
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, hit: &VisibleHit) -> Option<(Ray, Color)> {
        let reflected = r.direction().reflect(hit.normal);
        let attenuation = self.albedo;

        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if reflected.dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
