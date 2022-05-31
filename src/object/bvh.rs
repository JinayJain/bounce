// https://gfxcourses.stanford.edu/cs248/winter21content/media/acceleration/09_acceleration.pdf

use std::{
    ops::{Bound, Range},
    sync::Arc,
};

use crate::{
    color::Color,
    geometry::{Ray, Vec3},
    material::Lambertian,
};

use super::{Hit, HitRecord};

pub struct BoundingBox {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

impl BoundingBox {
    pub fn new(x: Range<f64>, y: Range<f64>, z: Range<f64>) -> Self {
        Self { x, y, z }
    }
}

fn intersection(a: &Range<f64>, b: &Range<f64>) -> Option<Range<f64>> {
    assert!(a.start <= a.end);
    assert!(b.start <= b.end);

    if a.end < b.start || b.end < a.start {
        None
    } else {
        let start = a.start.max(b.start);
        let end = a.end.min(b.end);

        Some(start..end)
    }
}

fn solve_axis(origin: f64, rate: f64, axis_range: &Range<f64>) -> Option<Range<f64>> {
    // parallel to axis
    if rate == 0.0 {
        return if axis_range.contains(&origin) {
            Some(f64::NEG_INFINITY..f64::INFINITY)
        } else {
            None
        };
    }

    let first_t = (axis_range.start - origin) / rate;
    let second_t = (axis_range.end - origin) / rate;

    let start = first_t.min(second_t);
    let end = first_t.max(second_t);

    Some(start..end)
}

macro_rules! unwrap_or_return {
    ($e: expr) => {
        match $e {
            Some(x) => x,
            None => return None,
        }
    };
}

impl Hit for BoundingBox {
    fn hit(&self, r: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let t_range_x = solve_axis(r.origin().x(), r.direction().x(), &self.x);
        let t_range_y = solve_axis(r.origin().y(), r.direction().y(), &self.y);
        let t_range_z = solve_axis(r.origin().z(), r.direction().z(), &self.z);

        let t_range_x = unwrap_or_return!(t_range_x);
        let t_range_y = unwrap_or_return!(t_range_y);
        let t_range_z = unwrap_or_return!(t_range_z);

        // combine all possible range of t to find the t's that satify all
        let combined = intersection(&t_range_x, &t_range_y)
            .and_then(|combined| intersection(&combined, &t_range_z))
            .and_then(|combined| intersection(&combined, &t_range));

        let combined = unwrap_or_return!(combined);

        let t = combined.start;
        Some(HitRecord::new(
            r,
            r.at(t),
            Vec3::new(0.0, 1.0, 0.0), // TODO: Implement normal for bounding box
            t,
            Arc::new(Lambertian::new(Color::new(0.0, 1.0, 0.0))), // TODO: Implement non-visible Hit for internal shapes like a bounding box
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_axis_basic() {
        let orig = 3.0;
        let rate = -2.0;

        let range = -1.0..3.0;

        let solved = solve_axis(orig, rate, &range);

        assert_eq!(solved.unwrap(), 0.0..2.0);
    }

    #[test]
    fn solve_axis_parallel_inside() {
        let orig = 1.0;
        let rate = 0.0;

        let range = -1.0..3.0;

        let solved = solve_axis(orig, rate, &range);

        assert_eq!(solved.unwrap(), f64::NEG_INFINITY..f64::INFINITY);
    }

    #[test]
    fn solve_axis_parallel_outside() {
        let orig = 8.0;
        let rate = 0.0;

        let range = -8.3..0.3;

        let solved = solve_axis(orig, rate, &range);

        assert!(solved.is_none());
    }

    #[test]
    fn intersections() {
        let a = 0.0..2.0;
        let b = -2.0..3.0;

        assert_eq!(intersection(&a, &b).unwrap(), a);
    }
}
