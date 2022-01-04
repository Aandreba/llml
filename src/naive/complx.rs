use std::ops::{Add, Sub};
use crate::others::Complx;

macro_rules! impl_inverse {
    ($trait:ident, $fn:ident, $symbol:tt, $($target:ident),*) => {
        $(
            impl $trait<Complx<$target>> for $target {
                type Output = Complx<$target>;
            
                fn $fn(self, rhs: Complx<$target>) -> Self::Output {
                    Complx::new(self $symbol rhs.re, self $symbol rhs.im)
                }
            }
        )*
    };
}

macro_rules! impl_arith {
    ($($trait:ident, $fn:ident, $symbol:tt),+) => {
        $(
            impl<T: $trait> $trait for Complx<T> {
                type Output = Complx<<T as $trait>::Output>;
            
                fn $fn(self, rhs: Self) -> Self::Output {
                    Complx::new(self.re $symbol rhs.re, self.im $symbol rhs.im)
                }
            }

            impl<T: $trait + Clone> $trait<T> for Complx<T> {
                type Output = Complx<<T as $trait>::Output>;
            
                fn $fn(self, rhs: T) -> Self::Output {
                    Complx::new(self.re $symbol rhs.clone(), self.im $symbol rhs)
                }
            }

            impl_inverse!($trait, $fn, $symbol, u8, u16, u32, u64, u128);
            impl_inverse!($trait, $fn, $symbol, i8, i16, i32, i64, i128);
            impl_inverse!($trait, $fn, $symbol, f32, f64);
        )*
    };
}

impl_arith!(
    Add, add, +,
    Sub, sub, -
);