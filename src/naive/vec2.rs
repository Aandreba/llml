use std::ops::{Add, Sub, Mul, Div};
use crate::{EucVec2};

macro_rules! impl_inverse {
    ($trait:ident, $fn:ident, $symbol:tt, $($target:ident),*) => {
        $(
            impl $trait<EucVec2<$target>> for $target {
                type Output = EucVec2<$target>;
            
                fn $fn(self, rhs: EucVec2<$target>) -> Self::Output {
                    EucVec2::new(self $symbol rhs.x, self $symbol rhs.y)
                }
            }
        )*
    };
}

macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl<T: $trait> $trait for EucVec2<T> {
            type Output = EucVec2<<T as $trait>::Output>;
        
            fn $fn(self, rhs: Self) -> Self::Output {
                EucVec2::new(self.x $symbol rhs.x, self.y $symbol rhs.y)
            }
        }

        impl<T: $trait + Clone> $trait<T> for EucVec2<T> {
            type Output = EucVec2<<T as $trait>::Output>;
        
            fn $fn(self, rhs: T) -> Self::Output {
                EucVec2::new(self.x $symbol rhs.clone(), self.y $symbol rhs)
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

impl<T> EucVec2<T> {
    /// Vector dot product
    pub fn dot (self, rhs: Self) -> <<T as Mul>::Output as Add>::Output where T: Mul, <T as Mul>::Output: Add {
        self.x * rhs.x + self.y * rhs.y
    }
}