x86_use!();
use cfg_if::cfg_if;

use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::vec::EucVecd2;

use super::{_mm_sum_ps, EucVecd4};

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecf4 (pub(crate) __m128);
impl_arith_sse!(EucVecf4, f32);

impl EucVecf4 {
    const DIV_MASK : __m128 = unsafe { *(&[u32::MAX, u32::MAX, u32::MAX, u32::MAX] as *const [u32;4] as *const __m128) };

    #[inline(always)]
    pub fn new (a: [f32;4]) -> Self {
        unsafe { Self(_mm_set_ps(a[3], a[2], a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scalar (x: f32) -> Self {
        unsafe { Self(_mm_set1_ps(x)) }
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

    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn sqrt (self) -> Self {
        unsafe { Self(_mm_sqrt_ps(self.0)) }
    }

    #[inline(always)]
    pub fn sqrt_fast (self) -> Self {
        unsafe { Self(_mm_rcp_ps(_mm_rsqrt_ps(self.0))) }
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVecd4> for EucVecf4 {
    #[inline(always)]
    fn into (self) -> EucVecd4 {
        cfg_if! {
            if #[cfg(target_feature = "avx")] { 
                unsafe { EucVecd4(_mm256_cvtps_pd(self.0)) }
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

impl From<[f32;4]> for EucVecf4 {
    #[inline(always)]
    fn from(x: [f32;4]) -> Self {
        unsafe { Self(_mm_loadu_ps(&x as *const [f32;4] as *const f32)) }
    }
}