use std::{ops::Range, rc::Rc};

use crate::geometry::{Point, Ray, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
}

pub trait Hit {
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

/// Stores a list of references to Hit objects
pub struct HittableList {
    objects: Vec<Rc<dyn Hit>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hit>) {
        self.objects.push(object);
    }
}

impl Hit for HittableList {
    /// Returns the closest hit from hitting all elements in the list
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut closest_t = t_range.end;

        self.objects
            .iter()
            .filter_map(|x| {
                let hit = x.hit(r, t_range.start..closest_t);

                if let Some(ref record) = hit {
                    closest_t = f64::min(closest_t, record.t);
                }

                hit
            })
            .reduce(|acc, hit| if acc.t > hit.t { hit } else { acc })
    }
}
