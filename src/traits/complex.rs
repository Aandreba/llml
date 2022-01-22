use crate::others::{Complxd, Complxf};

macro_rules! impl_complx_type {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl ComplxType<$ty> for $target {
                fn re (&self) -> $ty {
                    <$target>::re(self)
                }

                fn im (&self) -> $ty {
                    <$target>::im(self)
                }
            }
        )*
    }
}

macro_rules! impl_sqrtc {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl ComplexSqrt for $ty {
                type Output = $target;

                #[inline(always)]
                fn sqrtc (self) -> Self::Output {
                    $target::sqrtc(self)
                }
            }
        )*
    };
}

macro_rules! impl_powc {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl ComplexPow<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn expi (self) -> Self::Output {
                    $target::expi(self)
                }

                #[inline(always)]
                fn powci (self, rhs: $ty) -> Self::Output {
                    $target::powci(self, rhs)
                }

                #[inline(always)]
                fn powc (self, rhs: $target) -> Self::Output {
                    self.powf(rhs.re()) * self.powci(rhs.im())
                }
            }
        )*
    };
}

macro_rules! impl_all {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl_complx_type!($target, $ty);
            impl_sqrtc!($target, $ty);
            impl_powc!($target, $ty);
        )*
    }
}

pub trait ComplxType<T>: Copy {
    fn re (&self) -> T;
    fn im (&self) -> T;
}

pub trait ComplexSqrt<T = Self> {
    type Output: ComplxType<T>;

    /// Computes the square root of the value, returning the complex result
    fn sqrtc (self) -> Self::Output;
}

pub trait ComplexPow<Rhs: ComplxType<T>, T = Self> {
    type Output: ComplxType<T>;

    /// Computes ```exp(self * i)```
    fn expi (self) -> Self::Output;

    /// Computes ```pow(self, rhs * i)```
    fn powci (self, rhs: T) -> Self::Output;

    /// Computes ```pow(self, rhs)```, where ```rhs``` is a complex number
    fn powc (self, rhs: Rhs) -> Self::Output;
}

impl_all!(
    Complxf, f32,
    Complxd, f64
);