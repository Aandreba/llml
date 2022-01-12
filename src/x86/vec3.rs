x86_use!();

use cfg_if::cfg_if;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::{_mm_sum_ps, EucVecd3, EucVecd2};

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecf3 (pub(crate) __m128);
impl_arith_sse!(EucVecf3, f32);

impl EucVecf3 {
    #[inline(always)]
    pub fn new (x: f32, y: f32, z: f32) -> Self {
        unsafe { Self(_mm_set_ps(0., z, y, x)) }
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
    pub fn z (&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(2, 2, 2, 2))) }
    }

    #[inline(always)]
    pub fn sum (self) -> f32 {
        unsafe { _mm_sum_ps(self.0) }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVecd3> for EucVecf3 {
    #[inline(always)]
    fn into (self) -> EucVecd3 {
        cfg_if! {
            if #[cfg(target_feature = "avx")] { 
                todo!()
            } else {
                unsafe { EucVecd3(EucVecd2(_mm_cvtps_pd(self.0)), self.z().into()) }
            }
        }
    }
}