use crate::vec::*;
use std::ops::*;

macro_rules! impl_from_array {
    ($ty:ty, $($target:ident, $len:literal),+) => {
        $(
            impl_from_array!($target, $ty, $len);
        )*
    };

    ($target:ident, $ty:ty, $len:literal) => {
        impl From<[$ty;$len]> for $target {
            #[inline(always)]
            fn from (x: [$ty;$len]) -> Self {
                $target::new(x)
            }
        }
    };
}

macro_rules! impl_default {
    ($ty:ty, $($target:ident, $len:literal),+) => {
        $(
            impl_default!($target, $ty, $len);
        )*
    };

    ($target:ident, $ty:ty, $len:literal) => {
        impl Default for $target {
            #[inline(always)]
            fn default () -> Self {
                Self::from_scal(<$ty>::default())
            }
        }
    }
}

macro_rules! impl_unzip2 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl $target {
                #[inline(always)]
                pub fn unzip (self) -> ($ty, $ty) {
                    (self.x(), self.y())
                }
            }
        )*
    }
}

macro_rules! impl_unzip3 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl $target {
                #[inline(always)]
                pub fn unzip (self) -> ($ty, $ty, $ty) {
                    (self.x(), self.y(), self.z())
                }
            }
        )*
    };
}

macro_rules! impl_unzip4 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl $target {
                #[inline(always)]
                pub fn unzip (self) -> ($ty, $ty, $ty, $ty) {
                    (self.x(), self.y(), self.z(), self.w())
                }
            }
        )*
    }
}

macro_rules! impl_all {
    ($ty:ty, $($target:ident, $len:literal),+) => {
        $(
            impl_from_array!($target, $ty, $len);
            impl_default!($target, $ty, $len);
            impl_assign!(
                $target, 
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul,
                DivAssign, div_assign, div
            );

            impl_assign!(
                1, $target, $ty,
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul,
                DivAssign, div_assign, div
            );
        )*
    };
}

impl_all!(
    f32,
    EucVecf2, 2,
    EucVecf3, 3,
    EucVecf4, 4
);

impl_all!(
    f64,
    EucVecd2, 2,
    EucVecd3, 3,
    EucVecd4, 4
);

impl_unzip2! (
    EucVecf2, f32,
    EucVecd2, f64
);

impl_unzip3! (
    EucVecf3, f32,
    EucVecd3, f64
);

impl_unzip4! (
    EucVecf4, f32,
    EucVecd4, f64
);