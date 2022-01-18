arm_use!();
use core::mem::transmute;
use std::ptr::{addr_of};
use std::{ops::{Add, Sub, Mul, Div, Neg}};
use crate::{EucVecd2, EucVecd4};

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
            pub fn new (a: [$ty;3]) -> Self {
                unsafe { Self(transmute([a[0], a[1]]), a[2]) }
            }

            #[inline]
            pub fn from_scalar (x: $ty) -> Self {
                Self::new([x, x, x])
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
            pub fn cross (self, rhs: Self) -> Self {
                let v1 = EucVecd4::new([self.y(), self.z(), self.x(), self.z()]);
                let v2 = EucVecd4::new([rhs.z(), rhs.x(), rhs.y(), rhs.y()]);
                let m1 = v1 * v2;

                let v1 = EucVecd2::new([self.x(), self.y()]);
                let v2 = EucVecd2::new([rhs.z(), rhs.x()]);
                let m2 = v1 * v2;
                
                let v1 = EucVecd3(m1.0, m1.z());
                let v2 = EucVecd3::new([m1.w(), m2.x(), m2.y()]);
                v1 - v2
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
impl_vec3!(EucVecf3, f32, q);

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct EucVecd3 (pub(crate) EucVecd2, pub(crate) f64);
impl Eq for EucVecd3 {}
impl_vec3_vs!(EucVecd3, f64, q);

impl Into<EucVecf3> for EucVecd3 {
    #[inline(always)]
    fn into(self) -> EucVecf3 {
        unsafe { 
            let xy = vcvt_f32_f64(self.0.0);
            let z = vld1_f32(&[self.1 as f32, 0.] as *const [f32;2] as *const f32);
            EucVecf3(vcombine_f32(xy, z))
        }
    }
}

impl Into<EucVecd3> for EucVecf3 {
    #[inline(always)]
    fn into(self) -> EucVecd3 {
        unsafe {
            let xy = vcvt_f64_f32(vget_low_f32(self.0));
            let z = self.z() as f64;
            EucVecd3(EucVecd2(xy), z)
        }
    }
}