use std::ops::Range;

use crate::geometry::{Point, Ray, Vec3};

pub struct HitRecord {
    pub point: Point<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord>;
}
