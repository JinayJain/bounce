#[macro_export]
macro_rules! impl_math {
    ($new: ty, $generic: tt) => {
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        impl<$generic> Add for $new
        where
            $generic: Add<Output = $generic>,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl<$generic> AddAssign for $new
        where
            $generic: AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl<$generic> Div for $new
        where
            $generic: Div<Output = $generic>,
        {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl<$generic> DivAssign for $new
        where
            $generic: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl<$generic> Mul for $new
        where
            $generic: Mul<Output = $generic>,
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl<$generic> MulAssign for $new
        where
            $generic: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl<$generic> Neg for $new
        where
            $generic: Neg<Output = $generic>,
        {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<$generic> Sub for $new
        where
            $generic: Sub<Output = $generic>,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl<$generic> SubAssign for $new
        where
            $generic: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
    };

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
