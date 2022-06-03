#[macro_export]
macro_rules! impl_math_generic {
    ($new: ty) => {
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        impl<T> Add for $new
        where
            T: Add<Output = T> + Copy,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl<T> AddAssign for $new
        where
            T: AddAssign + Copy,
        {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl<T> Div for $new
        where
            T: Div<Output = T> + Copy,
        {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl<T> Div<T> for $new
        where
            T: Div<Output = T> + Copy,
        {
            type Output = Self;

            fn div(self, other: T) -> Self {
                Self(self.0 / other)
            }
        }

        impl<T> DivAssign for $new
        where
            T: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl<T> Mul for $new
        where
            T: Mul<Output = T> + Copy,
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl<T> Mul<T> for $new
        where
            T: Mul<Output = T> + Copy,
        {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                Self(self.0 * other)
            }
        }

        impl<T> MulAssign for $new
        where
            T: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl<T> Neg for $new
        where
            T: Neg<Output = T> + Copy,
        {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<T> Sub for $new
        where
            T: Sub<Output = T> + Copy,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl<T> SubAssign for $new
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_math {
    ($new: ty) => {
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        impl Add for $new {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl AddAssign for $new {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl Div for $new {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl DivAssign for $new {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl Mul for $new {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl MulAssign for $new {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl Neg for $new {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl Sub for $new {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl SubAssign for $new {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
    };
}
