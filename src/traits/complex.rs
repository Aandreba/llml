use std::ops::Neg;
use crate::generics::{Complxf, Complxd};

use super::{Sqrt, Zero};

// MACRO DEFS
macro_rules! impl_complexable {
    ($($target:ident, $complx:ident),+) => {
        $(
            impl Complxable for $target {
                type Output = $complx;

                #[inline(always)]
                fn as_im (self) -> <Self as Complxable>::Output {
                    $complx::new($target::zero(), self)
                }
            }

            impl Into<$complx> for $target {
                #[inline(always)]
                fn into (self) -> $complx {
                    $complx::new(self, $target::zero())
                }
            }
        )*
    };
}

// TRAIT DEFS
pub trait Complxable: Zero + PartialOrd + Neg<Output = Self> + Into<<Self as Complxable>::Output> {
    type Output;

    fn as_im (self) -> <Self as Complxable>::Output;

    #[inline(always)]
    fn as_re (self) -> <Self as Complxable>::Output {
        self.into()
    }
}

pub trait ComplexSqrt: Complxable {
    type Output: Complxable;

    fn sqrtc (self) -> <<Self as ComplexSqrt>::Output as Complxable>::Output;
}

// TRAIT IMPLS
impl_complexable!(
    f32, Complxf,
    f64, Complxd
);

impl<T: Complxable + Sqrt> ComplexSqrt for T where <T as Sqrt>::Output: Complxable {
    type Output = <T as Sqrt>::Output;

    #[inline(always)]
    fn sqrtc (self) -> <<Self as ComplexSqrt>::Output as Complxable>::Output {
        if self < Self::zero() {
            return (-self).sqrt().as_im()
        }

        self.sqrt().as_re()
    }
}