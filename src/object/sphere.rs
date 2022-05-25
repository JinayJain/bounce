use std::ops::Range;

use crate::geometry::{Point, Ray, Vec3};

use super::{Hit, HitRecord};

pub struct Sphere {
    center: Point<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point<f32>, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let direction = r.direction();
        let ray_origin = r.origin();

        let offset = Vec3::from(ray_origin - self.center);

        // Form quadratic for sphere intersection checking (simplified)
        let a = direction.len().powi(2);
        let half_b = direction.dot(offset);
        let c = offset.len().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        let sqrt_discrim = discriminant.sqrt();

        let plus_root = (-half_b + sqrt_discrim) / a;
        let minus_root = (-half_b - sqrt_discrim) / a;

        let root = if t_range.contains(&minus_root) {
            minus_root
        } else if t_range.contains(&plus_root) {
            plus_root
        } else {
            return None;
        };

        let hit_point = r.at(root);

        Some(HitRecord {
            t: root,
            point: hit_point,
            normal: Vec3::from(hit_point - self.center) / self.radius,
        })
    }
}
