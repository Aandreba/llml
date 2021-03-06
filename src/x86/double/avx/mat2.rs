x86_use!();

use cfg_if::cfg_if;
use crate::{EucVecd4, EucVecd2, traits::Zero};
use std::{ops::{Add, Sub, Mul, Div, Neg}};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Matd2 (pub(crate) EucVecd4);

impl Matd2 {
    pub fn new (a: [f64;4]) -> Self {
        Self(a.into())
    }

    #[inline(always)]
    pub fn tr (self) -> f64 {
        self.0.x() + self.0.w()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        unsafe {
            let v2 = _mm256_shuffle_pd(self.0.0, _mm256_setzero_pd(), _MM_SHUFFLE(0, 0, 2, 3));
            let m1 = EucVecd4(_mm256_mul_pd(self.0.0, v2));
            m1.x() - m1.y()
        }
    }

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        unsafe {
            let neg = EucVecd4(_mm256_sub_pd(_mm256_setzero_pd(), _mm256_shuffle_pd(self.0.0, self.0.0, _MM_SHUFFLE(0, 0, 2, 1))));
            Some(Self(EucVecd4::new(self.0.w(), neg.x(), neg.y(), self.0.x()) / det))
        }
    }

    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        let neg = EucVecd4(_mm256_sub_pd(_mm256_setzero_pd(), _mm256_shuffle_pd(self.0.0, self.0.0, _MM_SHUFFLE(0, 0, 2, 1))));
        Self(EucVecd4::new(self.0.w(), neg.x(), neg.y(), self.0.x()) / self.det())
    }
}


trait_map!(
    Matd2, f32,
    Add, add,
    Sub, sub
);

trait_map_scal!(
    Matd2, f32,
    Mul, mul,
    Div, div
);

impl Neg for Matd2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<EucVecd2> for Matd2 {
    type Output = EucVecd2;

    #[inline(always)]
    fn mul (self, rhs: EucVecd2) -> Self::Output {
        todo!()
    }
}

impl Mul for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 : __m128;
            let v3 : __m128;

            cfg_if! {
                if #[cfg(target_feature = "sse3")] {
                    v1 = _mm_moveldup_ps(self.0.0);
                    v3 = _mm_movehdup_ps(self.0.0);
                } else {
                    v1 = _mm_shuffle_ps(self.0.0, self.0.0, _MM_SHUFFLE(2, 2, 0, 0));
                    v3 = _mm_shuffle_ps(self.0.0, self.0.0, _MM_SHUFFLE(3, 3, 1, 1));
                }
            }
            
            let v2 = _mm_shuffle_ps(rhs.0.0, rhs.0.0, _MM_SHUFFLE(1, 0, 1, 0));
            let v4 = _mm_shuffle_ps(rhs.0.0, rhs.0.0, _MM_SHUFFLE(3, 2, 3, 2));

            let m1 = _mm_mul_ps(v1, v2);
            let m2 = _mm_mul_ps(v3, v4);
            Self(EucVecf4(_mm_add_ps(m1, m2)))
        }
    }
}