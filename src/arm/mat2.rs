use std::ptr::addr_of;
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};
use crate::traits::Zero;
use crate::{EucVecf2, EucVecf4, EucVecd2, EucVecd4, Complxf};
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
    ($($target:ident, $og:ident, $vec:ident, $ty:ident),+) => {
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
                pub fn tr (self) -> $ty {
                    self.0.x() + self.0.w()
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
                    Self($og::new(xx, zy, xy, yy))
                }
            }
        )*
    };
}

mat_wrap!(
    Matf2, EucVecf4, f32
);

impl_mat!(
    Matf2, EucVecf4, EucVecf2, f32
);

// FLOAT
impl Matf2 {
    #[inline(always)]
    pub fn from_cols (x: EucVecf2, y: EucVecf2) -> Self {
        unsafe { Self(EucVecf4(vcombine_f32(x.0, y.0))) }
    }

    #[inline(always)]
    pub fn col_x (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_low_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn col_y (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_high_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        unsafe {
            let x = vget_low_f32(self.0.0);
            let y = vget_high_f32(self.0.0);
            Self(EucVecf4(vcombine_f32(vtrn1_f32(x, y), vtrn2_f32(x, y))))
        }
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        unsafe {
            let x = vget_low_f32(self.0.0);
            let y = vget_high_f32(self.0.0);
    
            let v1 = vtrn1_f32(x, y);
            let v2 = vrev64_f32(vtrn2_f32(x, y));
            let v2 = vset_lane_f32(-vdups_lane_f32(v2, 1), v2, 1);

            EucVecf2(v1).dot(EucVecf2(v2))
        }
    }

    //#[inline(always)]
    pub fn inv (self) -> Option<Self> {
        unsafe {
            let x = vget_low_f32(self.0.0);
            let y = vget_high_f32(self.0.0);

            let v1 = vtrn1_f32(x, y);
            let v2 = vrev64_f32(vtrn2_f32(x, y));

            let neg_c = -vdups_lane_f32(v2, 1);
            let v2 = vset_lane_f32(neg_c, v2, 1);

            let det = EucVecf2(v1).dot(EucVecf2(v2));
            if det.is_zero() {
                return None
            }

            Some(Matf2::new(
                vdups_lane_f32(v2, 0), 
                -vdups_lane_f32(v1, 1),
                neg_c,
                vdups_lane_f32(v1, 0)
            ) / det)
        }
    }
}

impl Mul<EucVecf2> for Matf2 {
    type Output = EucVecf2;

    #[inline(always)]
    fn mul (self, rhs: EucVecf2) -> Self::Output {
        unsafe {
            let mul = self.0 * EucVecf4(vcombine_f32(vld1_dup_f32(&rhs.x()), vld1_dup_f32(&rhs.y())));
            let m1 = vget_low_f32(mul.0);
            let m2 = vget_high_f32(mul.0);

            EucVecf2(m1) + EucVecf2(m2)
        }
    }
}

impl Mul for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        let x = self * rhs.col_x();
        let y = self * rhs.col_y();

        unsafe {
            Self(EucVecf4(vcombine_f32(x.0, y.0)))
        }
    }
}