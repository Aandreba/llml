use std::ops::{Add, Sub, Mul, Div};
use crate::{EucVec3};

macro_rules! impl_inverse {
    ($trait:ident, $fn:ident, $symbol:tt, $($target:ident),*) => {
        $(
            impl $trait<EucVec3<$target>> for $target {
                type Output = EucVec3<$target>;
            
                fn $fn(self, rhs: EucVec3<$target>) -> Self::Output {
                    EucVec3::new(self $symbol rhs.x, self $symbol rhs.y, self $symbol rhs.z)
                }
            }
        )*
    };
}
macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl<T: $trait> $trait for EucVec3<T> {
            type Output = EucVec3<<T as $trait>::Output>;
        
            fn $fn(self, rhs: Self) -> Self::Output {
                EucVec3::new(self.x $symbol rhs.x, self.y $symbol rhs.y, self.z $symbol rhs.z)
            }
        }

        impl<T: $trait + Clone> $trait<T> for EucVec3<T> {
            type Output = EucVec3<<T as $trait>::Output>;
        
            fn $fn(self, rhs: T) -> Self::Output {
                EucVec3::new(self.x $symbol rhs.clone(), self.y $symbol rhs.clone(), self.z $symbol rhs)
            }
        }

        impl_inverse!($trait, $fn, $symbol, u8, u16, u32, u64, u128);
        impl_inverse!($trait, $fn, $symbol, i8, i16, i32, i64, i128);
        impl_inverse!($trait, $fn, $symbol, f32, f64);
    };
}

// ADDITION
impl_arith!(Add, add, +);
impl_arith!(Sub, sub, -);
impl_arith!(Mul, mul, *);
impl_arith!(Div, div, /);

impl<T> EucVec3<T> {
    /// Vector dot product
    pub fn dot (self, rhs: Self) -> T where T: Mul<T, Output = T> + Add<T, Output = T> {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}