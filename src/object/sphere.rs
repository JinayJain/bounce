use std::{ops::Range, sync::Arc};

use crate::{
    geometry::{Point, Ray, Vec3},
    material::Material,
};

use super::{
    bvh::{Bounded, BoundingBox, Primitive},
    Visible, VisibleHit,
};

pub struct Sphere {
    center: Point<f64>,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point<f64>, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Visible for Sphere {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        let direction = r.direction();
        let ray_origin = r.origin();

        let offset = Vec3::from(ray_origin - self.center);

        // Form quadratic for sphere intersection checking (simplified)
        let a = direction.len_sq();
        let half_b = direction.dot(offset);
        let c = offset.len_sq() - self.radius.powi(2);

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

        Some(VisibleHit::new(
            r,
            hit_point,
            Vec3::from(hit_point - self.center) / self.radius,
            root,
            Arc::clone(&self.material),
        ))
    }
}

impl Bounded for Sphere {
    fn bbox(&self) -> BoundingBox {
        let center = self.center;
        let first = Point::new(self.radius, self.radius, self.radius);
        let second = -first;

        BoundingBox::from_points(&[first + center, second + center])
    }

    fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }

    fn centroid(&self) -> Point<f64> {
        self.center
    }
}

impl Primitive for Sphere {}
