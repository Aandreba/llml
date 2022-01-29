x86_use!();
use crate::vec::EucVec2d;
use super::{_mm_sum_ps, EucVec3d};
use cfg_if::cfg_if;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[repr(transparent)]
pub struct EucVec3f (pub(crate) __m128);
impl_arith_sse!(EucVec3f, f32);

impl EucVec3f {
    const DIV_MASK : __m128 = unsafe { *(&[u32::MAX, u32::MAX, u32::MAX, 0] as *const [u32;4] as *const __m128) };
    const ABS_MASK : __m128 = unsafe { *(&[i32::MAX, i32::MAX, i32::MAX, 0] as *const [i32;4] as *const __m128) };

    #[inline(always)]
    pub fn new (a: [f32;3]) -> Self {
        unsafe { Self(_mm_set_ps(0., a[2], a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scal (x: f32) -> Self {
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
    // from [here](http://threadlocalmutex.com/?p=8)
    pub fn cross (self, rhs: Self) -> Self {
        unsafe {
            let a_yzx = _mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE(3, 0, 2, 1));
            let b_yzx = _mm_shuffle_ps(rhs.0, rhs.0, _MM_SHUFFLE(3, 0, 2, 1));
            let c = _mm_sub_ps(_mm_mul_ps(self.0, b_yzx), _mm_mul_ps(a_yzx, rhs.0));
            Self(_mm_shuffle_ps(c, c, _MM_SHUFFLE(3, 0, 2, 1)))
        }
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

impl Into<[f32;3]> for EucVec3f {
    #[inline(always)]
    fn into (self) -> [f32;3] {
        unsafe { *(&self as *const Self as *const [f32;3]) }
    }
}

#[cfg(target_feature = "sse2")]
impl Into<EucVec3d> for EucVec3f {
    #[inline(always)]
    fn into (self) -> EucVec3d {
        cfg_if! {
            if #[cfg(all(feature = "llml_avx", target_feature = "avx"))] { 
                unsafe { EucVec3d(_mm256_cvtps_pd(self.0)) }
            } else {
                unsafe { EucVec3d(EucVec2d(_mm_cvtps_pd(self.0)), self.z().into()) }
            }
        }
    }
}