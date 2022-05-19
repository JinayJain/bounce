use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> MulAssign for Vec3<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T> Div for Vec3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> DivAssign for Vec3<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let v = Vec3::new(2.1, -5.4, 0.0);

        assert_eq!(v.x, 2.1);
        assert_eq!(v.y, -5.4);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn it_adds() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a += b;

        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 7.0);
        assert_eq!(a.z, 9.0);

        println!("{:?}", b);
    }

    #[test]
    fn it_subtracts() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a -= b;

        assert_eq!(a.x, -3.0);
        assert_eq!(a.y, -3.0);
        assert_eq!(a.z, -3.0);
    }

    #[test]
    fn it_multiplies() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a *= b;

        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, 10.0);
        assert_eq!(a.z, 18.0);
    }

    #[test]
    fn it_divides() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a /= b;

        assert_eq!(a.x, 1.0 / 4.0);
        assert_eq!(a.y, 2.0 / 5.0);
        assert_eq!(a.z, 3.0 / 6.0);
    }

    #[test]
    fn it_negates() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);

        a = -a;

        assert_eq!(a.x, -1.0);
        assert_eq!(a.y, -2.0);
        assert_eq!(a.z, -3.0);
    }

    #[test]
    fn can_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let dot = a.dot(&b);

        assert_eq!(dot, 32.0);
    }

    #[test]
    fn works_with_i32() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        let dot = a.dot(&b);

        assert_eq!(dot, 32);
    }
}
