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
    EucVec2f, 2,
    EucVec3f, 3,
    EucVec4f, 4
);

impl_all!(
    f64,
    EucVec2d, 2,
    EucVec3d, 3,
    EucVec4d, 4
);

impl_unzip2! (
    EucVec2f, f32,
    EucVec2d, f64
);

impl_unzip3! (
    EucVec3f, f32,
    EucVec3d, f64
);

impl_unzip4! (
    EucVec4f, f32,
    EucVec4d, f64
);