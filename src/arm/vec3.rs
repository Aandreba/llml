arm_use!();
use core::mem::transmute;
use std::ptr::{addr_of, addr_of_mut};
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};
use crate::EucVecd2;

macro_rules! impl_vec3_vs {
    ($target:ident, $ty:ident, $tag:ident) => {
        impl_vec3_vs!(
            $target, $ty, $tag,
            Add, add, +,
            Sub, sub, -,
            Mul, mul, *,
            Div, div, /
        );

        impl $target {
            #[inline]
            pub fn new (x: $ty, y: $ty, z: $ty) -> Self {
                unsafe { Self(transmute([x, y]), z) }
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                self.0.x()
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                self.0.y()
            }

            #[inline(always)]
            pub fn z (&self) -> $ty {
                self.1
            }

            #[inline(always)]
            pub fn sum (self) -> $ty {
                self.0.sum() + self.1
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline(always)]
            pub fn cross (self, rhs: Self) -> Self {
                todo!()
            }
        }

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }

        impl PartialEq for $target {
            #[inline(always)]
            fn eq (&self, rhs: &Self) -> bool {
                self.1 == rhs.1 && self.0 == rhs.0
            }
        }

        impl Into<[$ty;3]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;3] {
                [self.x(), self.y(), self.1]
            }
        }
    };

    ($target:ident, $ty:ident, $tag:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0 $sy rhs.0, self.1 $sy rhs.1)
                }
            }

            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0 $sy rhs, self.1 $sy rhs)
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self $sy rhs.0, self $sy rhs.1)
                }
            }
        )*
    };
}

wrap!(EucVecf3, float32x4_t);
impl_vec3!(EucVecf3, f32, q, u32);

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct EucVecd3 (pub(crate) EucVecd2, pub(crate) f64);
impl Eq for EucVecd3 {}

impl_vec3_vs!(EucVecd3, f64, q);