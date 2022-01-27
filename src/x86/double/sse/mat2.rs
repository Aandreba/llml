x86_use!();
use crate::{traits::Zero, mat::Mat2f, vec::EucVec2d};
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute};
use super::EucVec4d;

macro_rules! impl_mat2 {
    ($target:ident, $ty:ident) => {
        impl_mat2!(
            $target, $ty,
            Add, add,
            Sub, sub
        );

        impl_mat2_scal!(
            $target, $ty,
            Mul, mul,
            Div, div
        );

        impl $target {
            #[inline(always)]
            pub fn transp (self) -> Self {
                Self::new([self.xx(), self.yx(), self.xy(), self.yy()])
            }
        }
    };

    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0.$fun(rhs.0))
                }
            }

            impl_mat2_scal!($target, $ty, $trait, $fun);
        )*
    };
}

macro_rules! impl_mat2_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0.$fun(rhs))
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self.$fun(rhs.0))
                }
            }
        )*
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Mat2d (pub(crate) EucVec4d);
impl_mat2!(Mat2d, f64);

impl Mat2d {
    #[inline(always)]
    pub fn new (a: [f64;4]) -> Self {
        Self(EucVec4d::new(a))
    }

    #[inline(always)]
    pub fn x (&self) -> EucVec2d {
        self.0.0
    }

    #[inline(always)]
    pub fn xx (&self) -> f64 {
        self.0.x()
    }

    #[inline(always)]
    pub fn xy (&self) -> f64 {
        self.0.y()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVec2d {
        self.0.1
    }

    #[inline(always)]
    pub fn yx (&self) -> f64 {
        self.0.z()
    }

    #[inline(always)]
    pub fn yy (&self) -> f64 {
        self.0.w()
    }

    #[inline(always)]
    pub fn tr (self) -> f64 {
        self.0.x()  +self.0.w()
    }

    #[inline(always)]
    pub fn scal_mul (self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        let m1 : EucVec2d = self.0.0 * EucVec2d::new([self.0.1.y(), self.0.1.x()]);
        m1.x() - m1.y()
    }

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        let vec = -EucVec2d::new([self.0.y(), self.0.z()]);
        Some(Self(EucVec4d::new([self.0.w(), vec.x(), vec.y(), self.0.x()]) / det))
    }

    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        let vec = -EucVec2d::new([self.0.y(), self.0.z()]);
        Self(EucVec4d::new([self.0.w(), vec.x(), vec.y(), self.0.x()]) / self.det())
    }
}

impl Mul<EucVec2d> for Mat2d {
    type Output = EucVec2d;

    #[inline(always)]
    fn mul(self, rhs: EucVec2d) -> Self::Output {
        let mul = self.0 * EucVec4d(rhs, rhs);
        let v1 = EucVec2d::new([mul.y(), mul.w()]); // odd
        let v2 = EucVec2d::new([mul.x(), mul.z()]); // even
        v1 + v2
    }
}


impl Mul for Mat2d {
    type Output = Mat2d;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 : EucVec4d = EucVec4d::new([self.0.x(), self.0.x(), self.0.z(), self.0.z()]);
            let v2 : EucVec4d = EucVec4d(rhs.0.0, rhs.0.0);
            let m1 = v1 * v2;

            let v3 : EucVec4d = EucVec4d::new([self.0.y(), self.0.y(), self.0.w(), self.0.w()]);
            let v4 : EucVec4d = EucVec4d(rhs.0.1, rhs.0.1);
            let m2 = v3 * v4;

            Self(m1 + m2)
        }
    }
}

impl Neg for Mat2d {
    type Output = Mat2d;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Into<[f64;4]> for Mat2d {
    #[inline(always)]
    fn into(self) -> [f64;4] {
        self.0.into()
    }
}

impl Into<Mat2f> for Mat2d {
    #[inline(always)]
    fn into(self) -> Mat2f {
        Mat2f(self.0.into())
    }
}