use crate::{impl_math, impl_math_generic};

use super::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T>(Vec3<T>);

impl_math_generic!(Point<T>);

impl<T> Point<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point(Vec3::new(x, y, z))
    }
}

impl<T> From<Vec3<T>> for Point<T> {
    fn from(v: Vec3<T>) -> Self {
        Point(v)
    }
}

impl<T> From<Point<T>> for Vec3<T> {
    fn from(p: Point<T>) -> Self {
        p.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let p = Point::from(v);
        assert_eq!(p.0, v);
    }

    #[test]
    fn test_vec3_from_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::from(p);
        assert_eq!(v, p.0);
    }

    #[test]
    fn test_point_math() {
        let p = Point::new(1.0, 2.0, 3.0);
        let q = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p + q, Point::new(2.0, 4.0, 6.0));
        assert_eq!(p - q, Point::new(0.0, 0.0, 0.0));
        assert_eq!(p * q, Point::new(1.0, 4.0, 9.0));
        assert_eq!(p / q, Point::new(1.0, 1.0, 1.0));
        assert_eq!(-p, Point::new(-1.0, -2.0, -3.0));
    }
}
