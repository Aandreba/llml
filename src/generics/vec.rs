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

impl_from_array!(
    f32,
    EucVecf2, 2,
    EucVecf3, 3,
    EucVecf4, 4
);

impl_from_array!(
    f64,
    EucVecd2, 2,
    EucVecd3, 3,
    EucVecd4, 4
);