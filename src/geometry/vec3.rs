use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    coords: [T; 3],
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { coords: [x, y, z] }
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.coords[0]
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.coords[1]
    }

    #[inline(always)]
    pub fn z(&self) -> T {
        self.coords[2]
    }
}

impl Vec3<f64> {
    /// Computes the 3D Euclidean length of the vector
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    /// Computes the squared length of a vector, equivalent to `v.dot(v)`
    pub fn len_sq(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    /// Returns the unit-length representation of the Vec3
    pub fn unit(&self) -> Self {
        let len = self.len();
        Vec3::new(self.x() / len, self.y() / len, self.z() / len)
    }

    /// Computes the dot product between two vectors by multiplying their components
    pub fn dot(&self, other: Vec3<f64>) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Computes the cross product between two vectors by multiplying their components
    pub fn cross(&self, other: Vec3<f64>) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    /// Reflects the vector by a surface given by its normal vector
    pub fn reflect(&self, normal: Self) -> Self {
        self.clone() - normal * 2.0 * self.dot(normal)
    }

    /// Refracts the vector (should be unit) by the IOR ratio eta_ratio on the normal
    pub fn refract(&self, normal: Vec3<f64>, eta_ratio: f64) -> Vec3<f64> {
        let v = self.clone();
        let cos_theta = normal.dot(-v).min(1.0);
        let perpendicular = eta_ratio * (v + cos_theta * normal);
        let parallel = -((1.0 - perpendicular.len_sq()).abs().sqrt()) * normal;

        perpendicular + parallel
    }

    /// Generates a random vector where all components are in the half-open range [min, max)
    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn near_zero(&self) -> bool {
        const THRESHOLD: f64 = 1e-8;

        (self.x().abs() < THRESHOLD) && (self.y().abs() < THRESHOLD) && (self.z().abs() < THRESHOLD)
    }

    /// Generate a random vector that lies in the unit-radius sphere
    pub fn random_in_unit_sphere() -> Self {
        // keep generating vectors until they lie in the unit radius sphere (and not the surrounding cube)
        loop {
            let cand = Vec3::random(-1.0, 1.0);

            if cand.len_sq() <= 1.0 {
                return cand;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
        rhs * self
    }
}

impl Div<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn div(self, rhs: Vec3<f64>) -> Self::Output {
        Vec3::new(self / rhs.x(), self / rhs.y(), self / rhs.z())
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Self) {
        for (this, other) in self.coords.iter_mut().zip(other.coords) {
            *this += other;
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        for (this, other) in self.coords.iter_mut().zip(other.coords) {
            *this -= other;
        }
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        for (this, other) in self.coords.iter_mut().zip(other.coords) {
            *this *= other;
        }
    }
}

impl<T> Div for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        for (this, other) in self.coords.iter_mut().zip(other.coords) {
            *this /= other;
        }
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let v = Vec3::new(2.1, -5.4, 0.0);

        assert_eq!(v.x(), 2.1);
        assert_eq!(v.y(), -5.4);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_addition() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a += b;

        assert_eq!(a.x(), 5.0);
        assert_eq!(a.y(), 7.0);
        assert_eq!(a.z(), 9.0);
    }

    #[test]
    fn test_subtraction() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a -= b;

        assert_eq!(a.x(), -3.0);
        assert_eq!(a.y(), -3.0);
        assert_eq!(a.z(), -3.0);
    }

    #[test]
    fn test_multiplication() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a *= b;

        assert_eq!(a.x(), 4.0);
        assert_eq!(a.y(), 10.0);
        assert_eq!(a.z(), 18.0);
    }

    #[test]
    fn test_division() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a /= b;

        assert_eq!(a.x(), 1.0 / 4.0);
        assert_eq!(a.y(), 2.0 / 5.0);
        assert_eq!(a.z(), 3.0 / 6.0);
    }

    #[test]
    fn test_negation() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);

        a = -a;

        assert_eq!(a.x(), -1.0);
        assert_eq!(a.y(), -2.0);
        assert_eq!(a.z(), -3.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let dot = a.dot(b);

        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let cross = a.cross(b);

        assert_eq!(cross.x(), -3.0);
        assert_eq!(cross.y(), 6.0);
        assert_eq!(cross.z(), -3.0);
        assert_eq!(cross.dot(a), 0.0);
        assert_eq!(cross.dot(b), 0.0);
    }

    #[test]
    fn test_generic() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        let c = a * b;
        assert_eq!(c, Vec3::new(4, 10, 18));
    }
}
