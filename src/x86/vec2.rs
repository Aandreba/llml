x86_use!();
use std::{ops::{Add, Sub, Mul, Div, Neg}};
use super::EucVec2d;

#[repr(transparent)]
pub struct EucVec2f (pub(crate) __m128);
impl_arith_sse!(EucVec2f, f32);

impl EucVec2f {
    const DIV_MASK : __m128 = unsafe { *(&[u32::MAX, u32::MAX, 0, 0] as *const [u32;4] as *const __m128) };
    const ABS_MASK : __m128 = unsafe { *(&[i32::MAX, i32::MAX, 0, 0] as *const [i32;4] as *const __m128) };

    #[inline(always)]
    pub fn new (a: [f32;2]) -> Self {
        unsafe { Self(_mm_set_ps(0., 0., a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scal (x: f32) -> Self {
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

    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.x().hypot(self.y())
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

impl Into<[f32;2]> for EucVec2f {
    #[inline(always)]
    fn into (self) -> [f32;2] {
        unsafe { *(&self as *const Self as *const [f32;2]) }
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVec2d> for EucVec2f {
    #[inline(always)]
    fn into (self) -> EucVec2d {
        unsafe { EucVec2d(_mm_cvtps_pd(self.0)) }
    }
}