use super::Material;
use crate::{color::Color, geometry::Ray, object::VisibleHit};

pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ior: index_of_refraction,
        }
    }

    /// Computes the Shlick approximated reflectance of the material at an angle
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    // TODO: Verify that dielectric scatter was correctly implemented
    fn scatter(&self, r: Ray, hit: &VisibleHit) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let eta_ratio = if hit.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_dir = r.direction().unit();

        let cos_theta = hit.normal.dot(-unit_dir).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = eta_ratio * sin_theta > 1.0;

        let scatter_dir =
            if cannot_refract || Dielectric::reflectance(cos_theta, eta_ratio) > rand::random() {
                unit_dir.reflect(hit.normal)
            } else {
                unit_dir.refract(hit.normal, eta_ratio)
            };

        let scattered = Ray::new(hit.point, scatter_dir);

        Some((scattered, attenuation))
    }
}
