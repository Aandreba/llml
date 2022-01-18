arm_use!();
use core::mem::transmute;
use std::ptr::{addr_of, addr_of_mut};
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};
use crate::EucVecd2;

macro_rules! impl_vec4_vv {
    ($target:ident, $ty:ident, $tag:ident) => {
        impl_vec4_vv!(
            $target, $ty, $tag,
            Add, add, +,
            Sub, sub, -,
            Mul, mul, *,
            Div, div, /
        );

        impl $target {
            #[inline]
            pub fn new (a: [$ty;4]) -> Self {
                unsafe { Self(transmute([a[0], a[1]]), transmute([a[2], a[3]])) }
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
                self.1.x()
            }

            #[inline(always)]
            pub fn w (&self) -> $ty {
                self.1.y()
            }

            #[inline(always)]
            pub fn sum (self) -> $ty {
                self.0.sum() + self.1.sum()
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline(always)]
            #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
            pub fn norm2 (self) -> $ty {
                self.dot(self)
            }

            #[inline(always)]
            pub fn norm (self) -> $ty {
                self.dot(self).sqrt()
            }

            #[inline(always)]
            pub fn unit (self) -> Self {
                self / self.norm()
            }

            #[inline(always)]
            pub fn sqrt (self) -> Self {
                Self(self.0.sqrt(), self.1.sqrt())
            }

            #[inline(always)]
            pub fn sqrt_fast (self) -> Self {
                self.sqrt()
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

        impl Into<[$ty;4]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;4] {
                unsafe { transmute(self) }
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

wrap!(EucVecf4, float32x4_t);
impl_vec4!(EucVecf4, f32, q);

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct EucVecd4 (pub(crate) EucVecd2, pub(crate) EucVecd2);
impl Eq for EucVecd4 {}
impl_vec4_vv!(EucVecd4, f64, q);

impl Into<EucVecf4> for EucVecd4 {
    #[inline(always)]
    fn into (self) -> EucVecf4 {
        unsafe {
            let xy = vcvt_f32_f64(self.0.0);
            let zw = vcvt_f32_f64(self.1.0);
            EucVecf4(vcombine_f32(xy, zw))
        }
    }
}

impl Into<EucVecd4> for EucVecf4 {
    #[inline(always)]
    fn into(self) -> EucVecd4 {
        unsafe {
            let xy = vcvt_f64_f32(vget_low_f32(self.0));
            let zw = vcvt_f64_f32(vget_high_f32(self.0));
            EucVecd4(EucVecd2(xy), EucVecd2(zw))
        }
    }
}