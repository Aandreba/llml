x86_use!();
use std::{ops::Mul, intrinsics::transmute};
use crate::{generics::{Complxf, Complxd}, _mm_combine_ps, EucVecf2, _mm_low_high_ps, EucVecf4, EucVecd2, EucVecd4};

impl Mul for Complxf {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = _mm_shuffle_ps(self.0.0, self.0.0, _MM_SHUFFLE(1, 1, 0, 0));
            let v2 = _mm_combine_ps(rhs.0.0, _mm_set_ps(0., 0., rhs.re(), -rhs.im()));
            let m1 = _mm_mul_ps(v1, v2);

            let (v1, v2) = _mm_low_high_ps(m1);
            Self(EucVecf2(_mm_add_ps(v1, v2)))
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
            Self(EucVecd2(_mm_add_pd(m1, m2)))
        }
    }
}