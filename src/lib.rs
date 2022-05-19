pub mod color;
pub mod geo;

#[macro_export]
macro_rules! impl_math {
    ($t: ty) => {
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        impl Add for $t {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl Sub for $t {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl SubAssign for $t {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl Mul for $t {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl MulAssign for $t {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl Div for $t {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl DivAssign for $t {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl Neg for $t {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
}
