x86_use!();
use crate::{EucVecd2, EucVecf4};
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
#[repr(C, align(32))]
pub struct EucVecd4 (pub(crate) EucVecd2, pub(crate) EucVecd2);
impl_arith_x2!(EucVecd4);

impl EucVecd4 {
    #[inline(always)]
    pub fn new (x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(EucVecd2::new(x, y), EucVecd2::new(z, w))
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
        self.1.x()
    }

    #[inline(always)]
    pub fn w (&self) -> f64 {
        self.1.y()
    }

    #[inline(always)]
    pub fn sum (self) -> f64 {
        self.0.sum() + self.1.sum()
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }
}

impl Into<EucVecf4> for EucVecd4 {
    fn into (self) -> EucVecf4 {
        unsafe {
            EucVecf4(_mm_shuffle_ps(_mm_cvtpd_ps(self.0.0),  _mm_cvtpd_ps(self.1.0), _MM_SHUFFLE(1, 0, 1, 0)))
        }
    }
}