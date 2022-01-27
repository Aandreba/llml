x86_use!();
use crate::{traits::Zero, vec::EucVec2d, mat::Mat3f};
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute};
use super::{EucVec4d, EucVec3d};

macro_rules! impl_matd3 {
    () => {
        impl_matd3!(
            Add, add,
            Sub, sub
        );

        impl_matd3_scal!(
            Mul, mul,
            Div, div
        );
    };

    ($($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for Mat3d {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(
                        self.0.$fun(rhs.0),
                        self.1.$fun(rhs.1),
                        self.2.$fun(rhs.2)
                    )
                }
            }

            impl_matd3_scal!($trait, $fun);
        )*
    };
}

macro_rules! impl_matd3_scal {
    ($($trait:ident, $fun:ident),+) => {
        $(
            impl $trait<f64> for Mat3d {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: f64) -> Self::Output {
                    Self(
                        self.0.$fun(rhs),
                        self.1.$fun(rhs),
                        self.2.$fun(rhs)
                    )
                }
            }

            impl $trait<Mat3d> for f64 {
                type Output = Mat3d;

                #[inline(always)]
                fn $fun (self, rhs: Mat3d) -> Self::Output {
                    Mat3d(
                        self.$fun(rhs.0),
                        self.$fun(rhs.1),
                        self.$fun(rhs.2)
                    )
                }
            }
        )*
    };
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(64))]
pub struct Mat3d (
    pub(crate) EucVec4d, pub(crate) EucVec4d, 
    pub(crate) f64
);
impl_matd3!();
impl Eq for Mat3d {}

impl Neg for Mat3d {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mat3d {
    #[inline]
    pub fn new (a: [f64;9]) -> Self {
        Mat3d (
            EucVec4d::new([a[0], a[1], a[2], a[3]]),
            EucVec4d::new([a[4], a[5], a[6], a[7]]),
            a[8]
        )
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        Self::new([
            self.xx(), self.yx(), self.zx(),
            self.xy(), self.yy(), self.zy(),
            self.xz(), self.yz(), self.zz()
        ])
    }

    #[inline(always)]
    pub fn x (&self) -> EucVec3d {
        EucVec3d(self.0.0, self.0.z())
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
    pub fn xz (&self) -> f64 {
        self.0.z()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVec3d {
        EucVec3d::new([self.0.w(), self.1.x(), self.1.y()])
    }

    #[inline(always)]
    pub fn yx (&self) -> f64 {
        self.0.w()
    }

    #[inline(always)]
    pub fn yy (&self) -> f64 {
        self.1.x()
    }

    #[inline(always)]
    pub fn yz (&self) -> f64 {
        self.1.y()
    }

    #[inline(always)]
    pub fn z (&self) -> EucVec3d {
        EucVec3d(self.1.1, self.2)
    }

    #[inline(always)]
    pub fn zx (&self) -> f64 {
        self.1.z()
    }

    #[inline(always)]
    pub fn zy (&self) -> f64 {
        self.1.w()
    }

    #[inline(always)]
    pub fn zz (&self) -> f64 {
        self.2
    }

    #[inline(always)]
    pub fn scal_mul (self, rhs: Self) -> Self {
        Self (
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2
        )
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        Self (
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2
        )
    }

    #[inline(always)]
    pub fn tr (self) -> f64 {
        EucVec3d::new([self.xx(), self.yy(), self.zz()]).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        // Negation
        let neg = -self.0;

        // Subdets 1 & 2
        let v0 = EucVec4d::new([self.xx(), neg.y(), neg.x(), self.xy()]);
        let v1 = EucVec4d::new([self.yy(), self.yx(), self.yz(), self.yz()]);
        let v2 = EucVec4d::new([self.zz(), self.zz(), self.zy(), self.zx()]);

        let m1 = v0 * v1 * v2;
        let s1 = m1.0 + m1.1;

        // Subdet 3
        let v5 = EucVec2d::new([self.xz(), neg.z()]);
        let v6 = EucVec2d::new([self.yx(), self.yy()]);
        let v7 = EucVec2d::new([self.zy(), self.zx()]);

        let m2 = v5 * v6 * v7;
        s1.sum() + m2.sum()
    }

    /// Performs the inverse of the matrix, returning ```None``` if it doesn't have one
    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        unsafe { Some(self._inv(det)) }
    }

    /// Performs the inverse without checking if the determinant is zero.\
    /// The use of this method is prefered if you're certain the matrix has an inverse,
    /// since it could be faster
    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        self._inv(self.det())
    }

    #[inline(always)]
    unsafe fn _inv (self, det: f64) -> Self {
        // Section #1
        let v1 = EucVec4d::new([self.yy(), self.xz(), self.xy(), self.yz()]);
        let v2 = EucVec4d::new([self.zz(), self.zy(), self.yz(), self.zx()]);
        let m1 = v1 * v2;

        let v1 = EucVec4d::new([self.yz(), self.xy(), self.xz(), self.yx()]);
        let v2 = EucVec4d::new([self.zy(), self.zz(), self.yy(), self.zz()]);
        let m2 = v1 * v2;

        let s1 = (m1 - m2) / det;

        // Section #2
        let v1 = EucVec4d::new([self.xx(), self.xz(), self.yx(), self.xy()]);
        let v2 = EucVec4d::new([self.zz(), self.yx(), self.zy(), self.zx()]);
        let m1 = v1 * v2;

        let v1 = EucVec4d::new([self.xz(), self.xx(), self.yy(), self.xx()]);
        let v2 = EucVec4d::new([self.zx(), self.yz(), self.zx(), self.zy()]);
        let m2 = v1 * v2;

        let s2 = (m1 - m2) / det;

        // Section #3
        let v1 : EucVec2d = self.0.0 * EucVec2d::new([self.yy(), self.yx()]);
        let s3 = (v1.x() - v1.y()) / det;
        
        // Result
        Self(s1, s2, s3)
    }
}

impl Mul<EucVec3d> for Mat3d {
    type Output = EucVec3d;

    #[inline(always)]
    fn mul (self, rhs: EucVec3d) -> Self::Output {
        let m1 = self.0 * EucVec4d::new([rhs.x(), rhs.y(), rhs.z(), rhs.x()]);
        let m2 = self.1 * EucVec4d::new([rhs.y(), rhs.z(), rhs.x(), rhs.y()]);
        let m3 = self.2 * rhs.z();

        let s1 = EucVec3d(m1.0, m1.z()).sum();
        let s2 = EucVec3d(m2.0, m1.w()).sum();
        let s3 = EucVec3d(m2.1, m3).sum();
        EucVec3d::new([s1, s2, s3])
    }
}

// TODO Optimize\
// In particular, Vec -> Mat m3 multiplcation could be joined into a single EucVecd3
impl Mul for Mat3d {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let m1 = self * EucVec3d::new([rhs.xx(), rhs.yx(), rhs.zx()]);
        let m2 = self * EucVec3d::new([rhs.xy(), rhs.yy(), rhs.zy()]);
        let m3 = self * EucVec3d::new([rhs.xz(), rhs.yz(), rhs.zz()]);

        Self (
            EucVec4d::new([m1.x(), m2.x(), m3.x(), m1.y()]),
            EucVec4d::new([m2.y(), m3.y(), m1.z(), m2.z()]),
            m3.z()
        )
    }
}

impl Into<[f64;9]> for Mat3d {
    fn into(self) -> [f64;9] {
        unsafe { transmute([Into::<[f64;3]>::into(self.x()), self.y().into(), self.z().into()]) }
    }
}

impl Into<Mat3f> for Mat3d {
    #[inline(always)]
    fn into(self) -> Mat3f {
        Mat3f(self.x().into(), self.y().into(), self.z().into())
    }
}