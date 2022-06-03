use std::{ops::Range, sync::Arc};

use crate::{
    geometry::{Point, Ray, Vec3},
    material::Material,
};

pub struct VisibleHit {
    pub point: Point<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,

    /// Introduce a private field to force users to use the new() function (can't bypass front_face calculation).
    /// Still allows for pub access of fields.
    ///
    /// Not the cleanest solution but it'll have to do for now.
    _force_new: (),
}

impl VisibleHit {
    pub fn new(
        r: Ray,
        point: Point<f64>,
        normal: Vec3<f64>,
        t: f64,
        material: Arc<dyn Material>,
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

pub trait Visible: Sync {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit>;
}

/// Stores a list of references to Hit objects
pub struct VisibleList {
    objects: Vec<Box<dyn Visible>>,
}

impl VisibleList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from_list(objects: Vec<Box<dyn Visible>>) -> Self {
        let mut list = Self::new();
        list.add_all(objects);

        list
    }

    pub fn add(&mut self, object: Box<dyn Visible>) {
        self.objects.push(object);
    }

    pub fn add_all(&mut self, mut objects: Vec<Box<dyn Visible>>) {
        self.objects.append(&mut objects);
    }
}

impl Visible for VisibleList {
    /// Returns the closest hit from hitting all elements in the list
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        let mut closest_t = t_range.end;

        self.objects
            .iter()
            .filter_map(|x| {
                let hit = x.bounce(r, &(t_range.start..closest_t));

                if let Some(ref record) = hit {
                    closest_t = f64::min(closest_t, record.t);
                }

                hit
            })
            .reduce(|acc, hit| if acc.t > hit.t { hit } else { acc })
    }
}
