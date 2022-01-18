use std::{ops::Mul, intrinsics::transmute};
use crate::{Complxf, EucVecf2, EucVecd2, Complxd};
arm_use!();

impl Mul for Complxf {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = vcombine_f32(vtrn1_f32(self.0.0, self.0.0), vtrn2_f32(self.0.0, self.0.0));
            let v2 = vcombine_f32(rhs.0.0, transmute([-rhs.im(), rhs.re()]));
            let m1 = vmulq_f32(v1, v2);

            Self(EucVecf2(vadd_f32(vget_low_f32(m1), vget_high_f32(m1))))
        }
    }
}

impl Mul for Complxd {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let m1 = vmulq_f64(vtrn1q_f64(self.0.0, self.0.0), rhs.0.0);
            let m2 = vmulq_f64(vtrn2q_f64(self.0.0, self.0.0), transmute([-rhs.im(), rhs.re()]));
            Self(EucVecd2(vaddq_f64(m1, m2)))
        }
    }
}