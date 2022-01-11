use std::ops::{Add, Sub, Mul, Div};
use crate::traits::Complx;
use crate::{EucVecf2, EucVecd2};
arm_use!();

macro_rules! declare {
    ($($name:ident, $og:ident, $ty:ident, $($tag:ident)?),+) => {
        $(  
            #[derive(Debug, Clone, Copy)]
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);
            impl_arith!($name, $ty);

            impl $name {
                #[inline(always)]
                pub fn conj (self) -> Self {
                    unsafe {
                        Self($og(concat_idents!(vset, $($tag,)? _lane_, $ty)(-concat_idents!(vget, $($tag,)? _lane_, $ty)(self.0.0, 1), self.0.0, 1)))
                    }
                }
            }

            impl Complx<$ty> for $name {
                #[inline(always)]
                fn new (re: $ty, im: $ty) -> Self {
                    Self($og::new(re, im))
                }

                #[inline(always)]
                fn re (&self) -> $ty {
                    self.0.x()
                }

                #[inline(always)]
                fn im (&self) -> $ty {
                    self.0.y()
                }
            }
        )*
    };
}

macro_rules! impl_arith {
    ($($target:ident, $ty:ident),+) => {
        $(
            impl_arith!(
                $target, $ty,
                Add, add, +,
                Sub, sub, -
            );

            impl_arith_scal!(
                $target, $ty,
                Mul, mul, *,
                Div, div, /
            );
        )*
    };

    ($target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0 $sy rhs.0)
                }
            }

            impl_arith_scal!($target, $ty, $trait, $fun, $sy);
        )*
    };  
}

macro_rules! impl_arith_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0 $sy rhs)
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self $sy rhs.0)
                }
            }
        )*
    }; 
}

declare!(
    Complxf, EucVecf2, f32, ,
    Complxd, EucVecd2, f64, q
);

impl Mul for Complxf {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        todo!()
    }
}
