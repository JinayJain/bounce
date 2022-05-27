use std::{ops::Range, rc::Rc};

use crate::{
    geometry::{Point, Ray, Vec3},
    material::Material,
};

pub struct HitRecord {
    pub point: Point<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub front_face: bool,

    /// Introduce a private field to force users to use the new() function (can't bypass front_face calculation).
    /// Still allows for pub access of fields.
    ///
    /// Not the cleanest solution but it'll have to do for now.
    _force_new: (),
}

impl HitRecord {
    pub fn new(
        r: Ray,
        point: Point<f64>,
        normal: Vec3<f64>,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = r.direction().dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            point,
            normal,
            t,
            material,
            front_face,
            _force_new: (),
        }
    }
}

pub trait Hit {
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

/// Stores a list of references to Hit objects
pub struct HittableList {
    objects: Vec<Box<dyn Hit>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
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
