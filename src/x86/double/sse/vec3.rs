x86_use!();
use crate::{EucVecd2, EucVecf3, EucVecf2};
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
#[repr(C, align(32))]
pub struct EucVecd3 (pub(crate) EucVecd2, pub(crate) f64);
impl_arith_x2!(EucVecd3);

impl EucVecd3 {
    #[inline(always)]
    pub fn new (a: [f64;3]) -> Self {
        Self(EucVecd2::new([a[0], a[1]]), a[2])
    }

    #[inline(always)]
    pub fn x (&self) -> f64 {
        self.0.x()
    }

    #[inline(always)]
    pub fn y (&self) -> f64 {
        self.0.y()
    }

    #[inline(always)]
    pub fn z (&self) -> f64 {
        self.1
    }

    #[inline(always)]
    pub fn sum (self) -> f64 {
        self.0.sum() + self.1
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }

    #[inline(always)]
    #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
    pub fn norm2 (self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm (self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn unit (self) -> Self {
        self / self.norm()
    }
}

impl Into<EucVecf3> for EucVecd3 {
    fn into (self) -> EucVecf3 {
        let z : f32 = self.1 as f32;
        unsafe {
            let a = _mm_cvtpd_ps(self.0.0);
            EucVecf3(_mm_or_ps(a, _mm_set_ps(0., z, 0., 0.)))
        }
    }
}