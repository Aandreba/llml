x86_use!();
use crate::{traits::Zero, vec::EucVecd2, mat::Matf3};
use std::ops::{Add, Sub, Mul, Div, Neg};

use super::{EucVecd4, EucVecd3};

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
            impl $trait for Matd3 {
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
            impl $trait<f64> for Matd3 {
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

            impl $trait<Matd3> for f64 {
                type Output = Matd3;

                #[inline(always)]
                fn $fun (self, rhs: Matd3) -> Self::Output {
                    Matd3(
                        self.$fun(rhs.0),
                        self.$fun(rhs.1),
                        self.$fun(rhs.2)
                    )
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, align(64))]
pub struct Matd3 (
    pub(crate) EucVecd4, pub(crate) EucVecd4, 
    pub(crate) f64
);
impl_matd3!();
impl Eq for Matd3 {}

impl Neg for Matd3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Matd3 {
    #[inline]
    pub fn new (a: [f64;9]) -> Self {
        Matd3 (
            EucVecd4::new([a[0], a[1], a[2], a[3]]),
            EucVecd4::new([a[4], a[5], a[6], a[7]]),
            a[8]
        )
    }

    /// Returns a matrix thet represents the specified rotation (in radians)
    pub fn of_rot (roll: f64, pitch: f64, yaw: f64) -> Self {
        let (sy, cy) = roll.sin_cos();
        let (sb, cb) = pitch.sin_cos();
        let (sa, ca) = yaw.sin_cos();

        let sbsy = sb * sy;
        let sbcy = sb * cy;

        Self::new([
            ca * cb, ca.mul_add(sbsy, -sa * cy), ca.mul_add(sbcy, sa * sy), 
            sa * cb, sa.mul_add(sbsy, ca * cy), sa.mul_add(sbcy, -ca * sy),
            -sb, cb * sy, cb * cy
        ])
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
    pub fn x (&self) -> EucVecd3 {
        EucVecd3(self.0.0, self.0.z())
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
    pub fn y (&self) -> EucVecd3 {
        EucVecd3::new([self.0.w(), self.1.x(), self.1.y()])
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
    pub fn z (&self) -> EucVecd3 {
        EucVecd3(self.1.1, self.2)
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
    pub fn tr (self) -> f64 {
        EucVecd3::new([self.xx(), self.yy(), self.zz()]).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        // Negation
        let neg = -self.0;

        // Subdets 1 & 2
        let v0 = EucVecd4::new([self.xx(), neg.y(), neg.x(), self.xy()]);
        let v1 = EucVecd4::new([self.yy(), self.yx(), self.yz(), self.yz()]);
        let v2 = EucVecd4::new([self.zz(), self.zz(), self.zy(), self.zx()]);

        let m1 = v0 * v1 * v2;
        let s1 = m1.0 + m1.1;

        // Subdet 3
        let v5 = EucVecd2::new([self.xz(), neg.z()]);
        let v6 = EucVecd2::new([self.yx(), self.yy()]);
        let v7 = EucVecd2::new([self.zy(), self.zx()]);

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
        let v1 = EucVecd4::new([self.yy(), self.xz(), self.xy(), self.yz()]);
        let v2 = EucVecd4::new([self.zz(), self.zy(), self.yz(), self.zx()]);
        let m1 = v1 * v2;

        let v1 = EucVecd4::new([self.yz(), self.xy(), self.xz(), self.yx()]);
        let v2 = EucVecd4::new([self.zy(), self.zz(), self.yy(), self.zz()]);
        let m2 = v1 * v2;

        let s1 = (m1 - m2) / det;

        // Section #2
        let v1 = EucVecd4::new([self.xx(), self.xz(), self.yx(), self.xy()]);
        let v2 = EucVecd4::new([self.zz(), self.yx(), self.zy(), self.zx()]);
        let m1 = v1 * v2;

        let v1 = EucVecd4::new([self.xz(), self.xx(), self.yy(), self.xx()]);
        let v2 = EucVecd4::new([self.zx(), self.yz(), self.zx(), self.zy()]);
        let m2 = v1 * v2;

        let s2 = (m1 - m2) / det;

        // Section #3
        let v1 : EucVecd2 = self.0.0 * EucVecd2::new([self.yy(), self.yx()]);
        let s3 = (v1.x() - v1.y()) / det;
        
        // Result
        Self(s1, s2, s3)
    }
}

impl Mul<EucVecd3> for Matd3 {
    type Output = EucVecd3;

    #[inline(always)]
    fn mul (self, rhs: EucVecd3) -> Self::Output {
        let m1 = self.0 * EucVecd4::new([rhs.x(), rhs.y(), rhs.z(), rhs.x()]);
        let m2 = self.1 * EucVecd4::new([rhs.y(), rhs.z(), rhs.x(), rhs.y()]);
        let m3 = self.2 * rhs.z();

        let s1 = EucVecd3(m1.0, m1.z()).sum();
        let s2 = EucVecd3(m2.0, m1.w()).sum();
        let s3 = EucVecd3(m2.1, m3).sum();
        EucVecd3::new([s1, s2, s3])
    }
}

// TODO Optimize\
// In particular, Vec -> Mat m3 multiplcation could be joined into a single EucVecd3
impl Mul for Matd3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let m1 = self * EucVecd3::new([rhs.xx(), rhs.yx(), rhs.zx()]);
        let m2 = self * EucVecd3::new([rhs.xy(), rhs.yy(), rhs.zy()]);
        let m3 = self * EucVecd3::new([rhs.xz(), rhs.yz(), rhs.zz()]);

        Self (
            EucVecd4::new([m1.x(), m2.x(), m3.x(), m1.y()]),
            EucVecd4::new([m2.y(), m3.y(), m1.z(), m2.z()]),
            m3.z()
        )
    }
}

impl Into<Matf3> for Matd3 {
    #[inline(always)]
    fn into(self) -> Matf3 {
        Matf3(self.x().into(), self.y().into(), self.z().into())
    }
}