use std::{ops::Range, sync::Arc};

use crate::{
    geometry::{Point, Ray, Vec3},
    material::Material,
};

use super::{
    bvh::{Bounded, BoundingBox, Primitive},
    Visible, VisibleHit,
};

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

impl Visible for InfinitePlane {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        let denom = self.normal.dot(r.direction());

        // ray direction is parallel to the plane
        if denom == 0.0 {
            return None;
        }

        let numer = self.normal.dot((self.origin - r.origin()).into());

        let t = numer / denom;

        if t_range.contains(&t) {
            Some(VisibleHit::new(
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

pub struct Tri {
    vertices: [Point<f64>; 3],
    normal: Vec3<f64>,
    material: Arc<dyn Material>,
}

impl Tri {
    pub fn new(a: Point<f64>, b: Point<f64>, c: Point<f64>, material: Arc<dyn Material>) -> Self {
        let normal = Vec3::from(b - a).cross((c - a).into());

        Self {
            vertices: [a, b, c],
            material,
            normal,
        }
    }
}

const EPSILON: f64 = 0.000001;
impl Visible for Tri {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        // implementation of the Möller–Trumbore ray-triangle intersection algorithm
        // variable names taken from: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

        let a = self.vertices[0];
        let b = self.vertices[1];
        let c = self.vertices[2];

        let e1 = Vec3::from(b - a);
        let e2 = Vec3::from(c - a);

        let h = r.direction().cross(e2);
        let dot = e1.dot(h);

        if dot.abs() < EPSILON {
            return None;
        }

        let dot_inv = 1.0 / dot;
        let s = Vec3::from(r.origin() - a);
        let u = dot_inv * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(e1);
        let v = dot_inv * r.direction().dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = dot_inv * e2.dot(q);

        if t_range.contains(&t) {
            Some(VisibleHit::new(
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

impl Bounded for Tri {
    fn bbox(&self) -> BoundingBox {
        BoundingBox::from_points(&self.vertices)
    }

    fn surface_area(&self) -> f64 {
        let (a, b, c) = (self.vertices[0], self.vertices[1], self.vertices[2]);

        Vec3::from(b - a).cross(Vec3::from(c - a)).len().abs() / 2.0
    }

    fn centroid(&self) -> Point<f64> {
        let inv = 1.0 / self.vertices.len() as f64;
        let normalize = Point::new(inv, inv, inv);

        self.vertices
            .clone()
            .into_iter()
            .reduce(|acc, item| acc + item)
            .unwrap()
            * normalize
    }
}

impl Primitive for Tri {}
