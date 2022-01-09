use std::ptr::addr_of;
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};
use crate::{EucVecf2, EucVecf4, EucVecd2, EucVecd4};
arm_use!();

macro_rules! impl_mat_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0 $sy rhs)
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self $sy rhs.0)
                }
            }
        )*
    }
}

macro_rules! impl_mat {
    ($($target:ident, $vec:ident, $ty:ident),+) => {
        $(
            impl_mat!(
                1, $target, $ty,
                Add, add, +,
                Sub, sub, -
            );

            impl_mat_scal!(
                $target, $ty,
                Mul, mul, *,
                Div, div, /
            );

            impl $target {
                #[inline(always)]
                pub fn transp (self) -> Self {
                    Self::new(
                        self.0.x(), self.0.z(), 
                        self.0.y(), self.0.w()
                    )
                }
            }

            impl Neg for $target {
                type Output = Self;

                #[inline(always)]
                fn neg (self) -> Self::Output {
                    Self(-self.0)
                }
            }
        )*
    };

    (1, $target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0 $sy rhs.0)
                }
            }

            impl_mat_scal!($target, $ty, $trait, $fun, $sy);
        )*
    };
}

macro_rules! mat_wrap {
    ($($target:ident, $og:ident, $ty:ident),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(transparent)]
            pub struct $target (pub(crate) $og);

            impl $target {
                #[inline]
                pub fn new (xx: $ty, xy: $ty, yx: $ty, yy: $ty) -> Self {
                    Self($og::new(xx, xy, yx, yy))
                }
            }
        )*
    };
}

mat_wrap!(
    Matf2, EucVecf4, f32,
    Matd2, EucVecd4, f64
);

impl_mat!(
    Matf2, EucVecf2, f32,
    Matd2, EucVecd2, f64
);

// FLOAT
impl Matf2 {
    #[inline(always)]
    pub fn x (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_low_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn y (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_high_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn tr (self) -> f32 {
        self.0.x() + self.0.w()
    }

    //[inline(always)]
    pub fn det (self) -> f32 {
        let m1 = self.x() * EucVecf2(unsafe { vrev64_f32(self.y().0) });
        m1.x() - m1.y()
    }

    #[inline(always)]
    pub fn inv (self) -> Self {
        let neg = -EucVecf2::new(self.0.y(), self.0.z());
        Self(EucVecf4::new(self.0.w(), neg.x(), neg.y(), self.0.x()) / self.det())
    }
}

impl Mul for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        let x = rhs.x();
        let y = rhs.y();

        unsafe {
            let v1 = vcombine_f32(vld1_dup_f32(&self.0.x()), vld1_dup_f32(&self.0.z()));
            let v2 = vcombine_f32(x.0, x.0);
            let m1 = EucVecf4(v1) * EucVecf4(v2);

            let v3 = vcombine_f32(vld1_dup_f32(&self.0.y()), vld1_dup_f32(&self.0.w()));
            let v4 = vcombine_f32(y.0, y.0);
            let m2 = EucVecf4(v3) * EucVecf4(v4);

            Self(m1 + m2)
        }
    }
}

impl Mul<EucVecf2> for Matf2 {
    type Output = EucVecf2;

    #[inline(always)]
    fn mul(self, rhs: EucVecf2) -> Self::Output {
        unsafe {
            EucVecf2::new(
                self.x().dot(rhs),
                self.y().dot(rhs)
            )
        }
    }
}

// DOUBLE
impl Matd2 {
    #[inline(always)]
    pub fn x (&self) -> EucVecd2 {
        self.0.0.clone()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVecd2 {
        self.0.1.clone()
    }
}