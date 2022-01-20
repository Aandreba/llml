x86_use!();

use crate::vec::EucVecd2;
use super::{_mm_sum_ps, EucVecd3};
use cfg_if::cfg_if;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecf3 (pub(crate) __m128);
impl_arith_sse!(EucVecf3, f32);

impl EucVecf3 {
    const DIV_MASK : __m128 = unsafe { *(&[u32::MAX, u32::MAX, u32::MAX, 0] as *const [u32;4] as *const __m128) };

    #[inline(always)]
    pub fn new (a: [f32;3]) -> Self {
        unsafe { Self(_mm_set_ps(0., a[2], a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scalar (x: f32) -> Self {
        unsafe { Self(_mm_set_ps(0., x, x, x)) }
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
impl Into<EucVecd3> for EucVecf3 {
    #[inline(always)]
    fn into (self) -> EucVecd3 {
        cfg_if! {
            if #[cfg(target_feature = "avx")] { 
                unsafe { EucVecd3(_mm256_cvtps_pd(self.0)) }
            } else {
                unsafe { EucVecd3(EucVecd2(_mm_cvtps_pd(self.0)), self.z().into()) }
            }
        }
    }
}