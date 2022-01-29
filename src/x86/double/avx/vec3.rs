x86_use!();
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::vec::{EucVec2d, EucVec3f};

#[repr(transparent)]
pub struct EucVec3d (pub(crate) __m256d);
impl_arith!(EucVec3d, f64, m256);

impl EucVec3d {
    const DIV_MASK : __m256d = unsafe { *(&[u64::MAX, u64::MAX, u64::MAX, 0] as *const [u64;4] as *const __m256d) };
    const ABS_MASK : __m256d = unsafe { *(&[i64::MAX, i64::MAX, i64::MAX, 0] as *const [i64;4] as *const __m256d) };

    #[inline(always)]
    pub fn new (a: [f64;3]) -> Self {
        unsafe { Self(_mm256_set_pd(0., a[2], a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scal (x: f64) -> Self {
        Self::new([x, x, x])
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
            EucVec2d(_mm_add_pd(vlow, vhigh)).sum()
        }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }

    #[inline(always)]
    // from [here](http://threadlocalmutex.com/?p=8)
    pub fn cross (self, rhs: Self) -> Self {
        unsafe {
            let a_yzx = _mm256_shuffle_pd(self.0, self.0, _MM_SHUFFLE(3, 0, 2, 1));
            let b_yzx = _mm256_shuffle_pd(rhs.0, rhs.0, _MM_SHUFFLE(3, 0, 2, 1));
            let c = _mm256_sub_pd(_mm256_mul_pd(self.0, b_yzx), _mm256_mul_pd(a_yzx, rhs.0));
            Self(_mm256_shuffle_pd(c, c, _MM_SHUFFLE(3, 0, 2, 1)))
        }
    }
}


impl PartialEq for EucVec3d {
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

impl Into<[f64;3]> for EucVec3d {
    #[inline(always)]
    fn into (self) -> [f64;3] {
        unsafe { *(&self as *const Self as *const [f64;3]) }
    }
}

impl Into<EucVec3f> for EucVec3d {
    #[inline(always)]
    fn into (self) -> EucVec3f {
        unsafe { EucVec3f(_mm256_cvtpd_ps(self.0)) }
    }
}