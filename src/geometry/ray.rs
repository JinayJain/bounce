use super::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point<f64>,
    direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point<f64>, direction: Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(self) -> Point<f64> {
        self.origin
    }

    pub fn direction(self) -> Vec3<f64> {
        self.direction
    }

    pub fn at(self, t: f64) -> Point<f64> {
        self.origin + (self.direction * t).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_near {
        ($x: expr, $y: expr, $d: expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn at_from_origin() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(2.3, -1.2, 5.3));

        let at = r.at(5.3);

        assert_near!(at.x(), 2.3 * 5.3, 1e-8);
        assert_near!(at.y(), -1.2 * 5.3, 1e-8);
        assert_near!(at.z(), 5.3 * 5.3, 1e-8);
    }

    #[test]
    fn at_from_non_origin() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vec3::new(2.3, -1.2, 5.3));

        let at = r.at(5.3);

        assert_near!(at.x(), 2.3 * 5.3 + 1.0, 1e-8);
        assert_near!(at.y(), -1.2 * 5.3 + 2.0, 1e-8);
        assert_near!(at.z(), 5.3 * 5.3 + 3.0, 1e-8);
    }

    #[test]
    fn zero_len_dir() {
        let r = Ray::new(Point::new(3.2, -2.3, 0.0), Vec3::new(0.0, 0.0, 0.0));

        let at = r.at(-1.0);

        assert_near!(at.x(), 3.2, 1e-8);
        assert_near!(at.y(), -2.3, 1e-8);
        assert_near!(at.z(), 0.0, 1e-8);
    }
}
