x86_use!();
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
#[repr(transparent)]
pub struct EucVecd2 (pub(crate) __m128d);
impl_arith_sse!(EucVecd2, f64);

impl EucVecd2 {
    #[inline(always)]
    pub fn new (x: f64, y: f64) -> Self {
        unsafe { Self(_mm_set_pd(y, x)) }
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
}