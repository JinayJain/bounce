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

const EPSILON: f64 = 1e-8;
impl Visible for Tri {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        // implementation of the Möller–Trumbore ray-triangle intersection algorithm
        // variable names taken from the original paper: https://cadxfem.org/inf/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf

        let e1 = Vec3::from(self.vertices[1] - self.vertices[0]);
        let e2 = Vec3::from(self.vertices[2] - self.vertices[0]);

        let p = r.direction().cross(e2);

        let det = p.dot(e1);

        // this can be changed in the future to allow for backface culling
        // right now, Tri's are double sided
        if det.abs() < EPSILON {
            return None;
        }

        let det_inv = 1.0 / det;

        let t_vec = Vec3::from(r.origin() - self.vertices[0]);
        let q = t_vec.cross(e1);

        let t = q.dot(e2) * det_inv;
        let u = p.dot(t_vec) * det_inv;
        let v = q.dot(r.direction()) * det_inv;

        if u < 0.0 || u > 1.0 || v < 0.0 || u + v > 1.0 {
            return None;
        }

        if t_range.contains(&t) {
            let point = r.at(t);

            Some(VisibleHit::new(
                r,
                point,
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
        BoundingBox::from_points(&self.vertices).unwrap()
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
