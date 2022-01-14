arm_use!();
use crate::{EucVecf3, EucVecf2, EucVecf4, Matf2, traits::Zero, EucVecd2, EucVecd3, EucVecd4};
use std::ops::{Add, Sub, Mul, Div, Neg};

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

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct Matd3 (
    pub(crate) EucVecd4, pub(crate) EucVecd4, 
    pub(crate) f64
);

impl_matd3!();

impl Matd3 {
    #[inline]
    pub fn new (a: [f64;9]) -> Self {
        Matd3 (
            EucVecd4::new(a[0], a[1], a[2], a[3]),
            EucVecd4::new(a[4], a[5], a[6], a[7]),
            a[8]
        )
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
        EucVecd3::new(self.0.w(), self.1.x(), self.1.y())
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
        EucVecd3::new(self.xx(), self.yy(), self.zz()).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        todo!()
    }

    /// Performs the inverse of the matrix, returning ```None``` if it doesn't have one
    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        todo!()
    }

    /// Performs the inverse without checking if the determinant is zero.\
    /// The use of this method is prefered if you're certain the matrix has an inverse,
    /// since it could be faster
    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        todo!()
    }
}

impl Mul<EucVecd3> for Matd3 {
    type Output = EucVecd3;

    #[inline(always)]
    fn mul (self, rhs: EucVecd3) -> Self::Output {
        let m1 = self.0 * EucVecd4::new(rhs.x(), rhs.y(), rhs.z(), rhs.x());
        let m2 = self.1 * EucVecd4::new(rhs.y(), rhs.z(), rhs.x(), rhs.y());
        let m3 = self.2 * rhs.z();

        let s1 = EucVecd3(m1.0, m1.z()).sum();
        let s2 = EucVecd3(m2.0, m1.w()).sum();
        let s3 = EucVecd3(m2.1, m3).sum();
        EucVecd3::new(s1, s2, s3)
    }
}

impl Mul for Matd3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}