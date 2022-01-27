x86_use!();
use std::{ops::Mul, intrinsics::transmute};
use crate::others::{Complxf, Complxd};
use super::{vec2::EucVec2f, _mm_combine_ps, _mm_low_high_ps, EucVec2d};

impl Mul for Complxf {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = _mm_shuffle_ps(self.0.0, self.0.0, _MM_SHUFFLE(1, 1, 0, 0));
            let v2 = _mm_combine_ps(rhs.0.0, _mm_set_ps(0., 0., rhs.re(), -rhs.im()));
            let m1 = _mm_mul_ps(v1, v2);

            let (v1, v2) = _mm_low_high_ps(m1);
            Self(EucVec2f(_mm_add_ps(v1, v2)))
        }
    }
}

impl Mul for Complxd {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let m1 = _mm_mul_pd(_mm_set1_pd(self.re()), rhs.0.0);
            let m2 = _mm_mul_pd(_mm_set1_pd(self.im()), transmute([-rhs.im(), rhs.re()]));
            Self(EucVec2d(_mm_add_pd(m1, m2)))
        }
    }
}