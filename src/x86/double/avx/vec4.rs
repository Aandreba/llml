x86_use!();
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute, ptr::addr_of};
use crate::vec::{EucVec2d, EucVec4f};

#[repr(transparent)]
pub struct EucVec4d (pub(crate) __m256d);
impl_arith!(EucVec4d, f64, m256);

impl EucVec4d {
    const DIV_MASK : __m256d = unsafe { *(&[u64::MAX, u64::MAX, u64::MAX, u64::MAX] as *const [u64;4] as *const __m256d) };
    const ABS_MASK : __m256d = unsafe { *(&[i64::MAX, i64::MAX, i64::MAX, i64::MAX] as *const [i64;4] as *const __m256d) };

    #[inline(always)]
    pub fn new (a: [f32;4]) -> Self {
        unsafe { 
            let vec = _mm256_load_pd(addr_of!(a).cast());
            Self(_mm256_shuffle_pd(vec, vec, _MM_SHUFFLE(0, 1, 2, 3)))
        }
    }

    #[inline(always)]
    pub fn from_scal (x: f64) -> Self {
        unsafe { Self(_mm256_set1_pd(x)) }
    }

    #[inline(always)]
    pub fn x (&self) -> f64 {
        unsafe { _mm256_cvtsd_f64(self.0) }
    }

    #[inline(always)]
    pub fn y (&self) -> f64 {
        unsafe { _mm256_cvtsd_f64(_mm256_shuffle_pd(self.0, self.0, _MM_SHUFFLE(1, 1, 1, 1))) }
    }

    #[inline(always)]
    pub fn z (&self) -> f64 {
        unsafe { _mm256_cvtsd_f64(_mm256_shuffle_pd(self.0, self.0, _MM_SHUFFLE(2, 2, 2, 2))) }
    }

    #[inline(always)]
    pub fn w (&self) -> f64 {
        unsafe { _mm256_cvtsd_f64(_mm256_shuffle_pd(self.0, self.0, _MM_SHUFFLE(3, 3, 3, 3))) }
    }

    #[inline(always)]
    pub fn sum (self) -> f64 {
        unsafe {
            let vlow  = _mm256_castpd256_pd128(self.0);
            let vhigh = _mm256_extractf128_pd(self.0, 1); // high 128
            EucVec2d(_mm_add_pd(vlow, vhigh)).sum()
        }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }
}


impl PartialEq for EucVec4d {
    #[inline(always)]
    fn eq (&self, rhs: &Self) -> bool {
        unsafe {
            let cmp = _mm256_cmp_pd(self.0, rhs.0, _MM_CMPINT_EQ);
            let low = *(&_mm256_castpd256_pd128(cmp) as *const __m128d as *const u128);
            let high = *(&_mm256_extractf128_pd(cmp, 1) as *const __m128d as *const u128);
            low == u128::MAX && high == u128::MAX
        }
    }
}

impl Into<[f64;4]> for EucVec4d {
    #[inline(always)]
    fn into (self) -> [f64;4] {
        unsafe { transmute(self.0) }
    }
}

impl Into<EucVec4f> for EucVec4d {
    #[inline(always)]
    fn into (self) -> EucVec4f {
        unsafe { EucVec4f(_mm256_cvtpd_ps(self.0)) }
    }
}