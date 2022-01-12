x86_use!();
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::EucVecd2;

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecd3 (pub(crate) __m256d);
impl_arith!(EucVecd3, f64, m256);

impl EucVecd3 {
    #[inline(always)]
    pub fn new (x: f64, y: f64, z: f64) -> Self {
        unsafe { Self(_mm256_set_pd(0., z, y, x)) }
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
    pub fn sum (self) -> f64 {
        unsafe {
            let vlow  = _mm256_castpd256_pd128(self.0);
            let vhigh = _mm256_extractf128_pd(self.0, 1); // high 128
            EucVecd2(_mm_add_pd(vlow, vhigh)).sum()
        }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }
}


impl PartialEq for EucVecd3 {
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

impl Into<EucVecf3> for EucVecd3 {
    #[inline(always)]
    fn into (self) -> EucVecf3 {
        unsafe { EucVecf4(_mm256_cvtpd_ps(self.0)) }
    }
}