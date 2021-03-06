x86_use!();
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute};
use crate::x86::vec2::EucVecf2;

#[repr(transparent)]
pub struct EucVecd2 (pub(crate) __m128d);
impl_arith_sse!(EucVecd2, f64);

impl EucVecd2 {
    const DIV_MASK : __m128d = unsafe { *(&[u64::MAX, u64::MAX] as *const [u64;2] as *const __m128d) };
    const ABS_MASK : __m128d = unsafe { *(&[i64::MAX, i64::MAX] as *const [i64;2] as *const __m128d) };

    #[inline(always)]
    pub fn new (a: [f64;2]) -> Self {
        unsafe { Self(_mm_set_pd(a[1], a[0])) }
    }

    #[inline(always)]
    pub fn from_scal (x: f64) -> Self {
        unsafe { Self(_mm_set1_pd(x))  }
    }

    #[inline(always)]
    pub fn x (&self) -> f64 {
        unsafe { _mm_cvtsd_f64(self.0) }
    }

    #[inline(always)]
    pub fn y (&self) -> f64 {
        unsafe { _mm_cvtsd_f64(_mm_shuffle_pd(self.0, self.0, _MM_SHUFFLE(1, 1, 1, 1))) }
    }

    #[inline(always)]
    pub fn sum (self) -> f64 {
        self.x() + self.y()
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }

    #[inline(always)]
    pub fn norm (self) -> f64 {
        self.x().hypot(self.y())
    }

    #[inline(always)]
    pub fn abs (self) -> Self {
        unsafe { Self(_mm_and_pd(Self::ABS_MASK, self.0)) }
    }
    
    #[inline(always)]
    pub fn sqrt (self) -> Self {
        unsafe { Self(_mm_sqrt_pd(self.0)) }
    }

    #[inline(always)]
    pub fn sqrt_fast (self) -> Self {
        self.sqrt()
    }
}

impl Into<[f64;2]> for EucVecd2 {
    #[inline(always)]
    fn into (self) -> [f64;2] {
        unsafe { transmute(self.0) }
    }
}

impl Into<EucVecf2> for EucVecd2 {
    #[inline(always)]
    fn into(self) -> EucVecf2 {
        unsafe { EucVecf2(_mm_cvtpd_ps(self.0)) }
    }
}