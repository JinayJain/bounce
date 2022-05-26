use crate::{color::Color, geometry::Ray, object::HitRecord};

use super::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r.direction().reflect(hit.normal);
        let attenuation = self.albedo;

        let scattered = Ray::new(hit.point, reflected);

        if reflected.dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
