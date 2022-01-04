use std::ops::{Add, Sub, Mul, Div};
use crate::vec::{EucVec4};

macro_rules! impl_inverse {
    ($trait:ident, $fn:ident, $symbol:tt, $($target:ident),*) => {
        $(
            impl $trait<EucVec4<$target>> for $target {
                type Output = EucVec4<$target>;
            
                fn $fn(self, rhs: EucVec4<$target>) -> Self::Output {
                    EucVec4::new(self $symbol rhs.x, self $symbol rhs.y, self $symbol rhs.z, self $symbol rhs.w)
                }
            }
        )*
    };
}
macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl<T: $trait> $trait for EucVec4<T> {
            type Output = EucVec4<<T as $trait>::Output>;
        
            fn $fn(self, rhs: Self) -> Self::Output {
                EucVec4::new(self.x $symbol rhs.x, self.y $symbol rhs.y, self.z $symbol rhs.z, self.w $symbol rhs.w)
            }
        }

        impl<T: $trait + Clone> $trait<T> for EucVec4<T> {
            type Output = EucVec4<<T as $trait>::Output>;
        
            fn $fn(self, rhs: T) -> Self::Output {
                EucVec4::new(self.x $symbol rhs.clone(), self.y $symbol rhs.clone(), self.z.clone() $symbol rhs.clone(), self.w $symbol rhs)
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

impl<T> EucVec4<T> {
    /// Vector dot product
    pub fn dot (self, rhs: Self) -> T where T: Mul<T, Output = T> + Add<T, Output = T> {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}