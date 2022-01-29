x86_use!();
use cfg_if::cfg_if;
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute, ptr::addr_of};
use crate::vec::EucVec2d;
use super::{_mm_sum_ps, EucVec4d};

#[repr(transparent)]
pub struct EucVec4f (pub(crate) __m128);
impl_arith_sse!(EucVec4f, f32);

impl EucVec4f {
    const DIV_MASK : __m128 = unsafe { *(&[u32::MAX, u32::MAX, u32::MAX, u32::MAX] as *const [u32;4] as *const __m128) };
    const ABS_MASK : __m128 = unsafe { *(&[i32::MAX, i32::MAX, i32::MAX, i32::MAX] as *const [i32;4] as *const __m128) };

    #[inline(always)]
    pub fn new (a: [f32;4]) -> Self {
        unsafe { Self(_mm_load_ps(addr_of!(a).cast())) }
    }

    #[inline(always)]
    pub fn from_scal (x: f32) -> Self {
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
    pub fn abs (self) -> Self {
        unsafe { Self(_mm_and_ps(Self::ABS_MASK, self.0)) }
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

impl Into<[f32;4]> for EucVec4f {
    #[inline(always)]
    fn into (self) -> [f32;4] {
        unsafe { transmute(self.0) }
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVec4d> for EucVec4f {
    #[inline(always)]
    fn into (self) -> EucVec4d {
        cfg_if! {
            if #[cfg(all(feature = "llml_avx", target_feature = "avx"))] { 
                unsafe { EucVec4d(_mm256_cvtps_pd(self.0)) }
            } else {
                unsafe { 
                    EucVec4d(
                        EucVec2d(_mm_cvtps_pd(self.0)), 
                        EucVec2d(_mm_cvtps_pd(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(1, 0, 3, 2))))
                    ) 
                }
            }
        }
    }
}