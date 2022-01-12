x86_use!();
use cfg_if::cfg_if;

use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::{_mm_sum_ps, EucVecd4, EucVecd2};

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecf4 (pub(crate) __m128);
impl_arith_sse!(EucVecf4, f32);

impl EucVecf4 {
    #[inline(always)]
    pub fn new (x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { Self(_mm_set_ps(w, z, y, x)) }
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
    pub fn w (&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(3, 3, 3, 3))) }
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
impl Into<EucVecd4> for EucVecf4 {
    #[inline(always)]
    fn into (self) -> EucVecd4 {
        cfg_if! {
            if #[cfg(target_feature = "avx")] { 
                todo!()
            } else {
                unsafe { 
                    EucVecd4(
                        EucVecd2(_mm_cvtps_pd(self.0)), 
                        EucVecd2(_mm_cvtps_pd(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(1, 0, 3, 2))))
                    ) 
                }
            }
        }
    }
}