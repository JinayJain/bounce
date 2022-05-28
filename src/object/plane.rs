use std::{ops::Range, sync::Arc};

use crate::{
    geometry::{Point, Ray, Vec3},
    material::Material,
};

use super::{Hit, HitRecord};

pub struct InfinitePlane {
    normal: Vec3<f64>,
    origin: Point<f64>,
    material: Arc<dyn Material>,
}

impl InfinitePlane {
    pub fn new(origin: Point<f64>, normal: Vec3<f64>, material: Arc<dyn Material>) -> Self {
        let normal = normal.unit();
        Self {
            origin,
            normal,
            material,
        }
    }
}

impl Hit for InfinitePlane {
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction());

        // ray direction is parallel to the plane
        if denom == 0.0 {
            return None;
        }

        let numer = self.normal.dot((self.origin - r.origin()).into());

        let t = numer / denom;

        if t_range.contains(&t) {
            Some(HitRecord::new(
                r,
                r.at(t),
                self.normal,
                t,
                Arc::clone(&self.material),
            ))
        } else {
            None
        }
    }
}
