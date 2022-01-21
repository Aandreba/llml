use crate::vec::*;

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

macro_rules! impl_all {
    ($ty:ty, $($target:ident, $len:literal),+) => {
        $(
            impl_from_array!($target, $ty, $len);
            impl_default!($target, $ty, $len);
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
