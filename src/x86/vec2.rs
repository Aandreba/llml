x86_use!();
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::EucVecd2;

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecf2 (pub(crate) __m128);
impl_arith_sse!(EucVecf2, f32);

impl EucVecf2 {
    #[inline(always)]
    pub fn new (x: f32, y: f32) -> Self {
        unsafe { Self(_mm_set_ps(0., 0., y, x)) }
    }

    #[inline(always)]
    pub fn from_scalar (x: f32) -> Self {
        unsafe { Self(_mm_set_ps(0., 0., x, x)) }
    }

    #[inline(always)]
    pub fn x (&self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    #[inline(always)]
    pub fn y (&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(1, 1, 1, 1))) }
    }

    #[inline(always)]
    pub fn sum (self) -> f32 {
        self.x() + self.y()
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVecd2> for EucVecf2 {
    #[inline(always)]
    fn into (self) -> EucVecd2 {
        unsafe { EucVecd2(_mm_cvtps_pd(self.0)) }
    }
}