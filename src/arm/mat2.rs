arm_use!();
use crate::{EucVecf4, EucVecf2, traits::Zero, EucVecd2};
use std::{ops::{Add, Sub, Mul, Div, Neg}};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Matf2 (pub(crate) EucVecf4);
impl_mat2!(Matf2, f32);

impl Matf2 {
    #[inline(always)]
    pub fn new (a: [f32;4]) -> Self {
        Self(EucVecf4::new(a))
    }

    #[inline(always)]
    pub fn x (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_low_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn xx (&self) -> f32 {
        self.0.x()
    }

    #[inline(always)]
    pub fn xy (&self) -> f32 {
        self.0.y()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVecf2 {
        unsafe { EucVecf2(vget_high_f32(self.0.0)) }
    }

    #[inline(always)]
    pub fn yx (&self) -> f32 {
        self.0.z()
    }

    #[inline(always)]
    pub fn yy (&self) -> f32 {
        self.0.w()
    }

    #[inline(always)]
    pub fn tr (self) -> f32 {
        self.0.x() + self.0.w()
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        unsafe {
            let v1 = vget_low_f32(self.0.0);
            let v2 = vrev64_f32(vget_high_f32(self.0.0));
            let m1 = EucVecf2(vmul_f32(v1, v2));

            m1.x() - m1.y()
        }
    }

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        let vec = -EucVecf2::new(self.0.y(), self.0.z());
        Some(Self(EucVecf4::new(self.0.w(), vec.x(), vec.y(), self.0.x()) / det))
    }

    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        let vec = -EucVecf2::new(self.0.y(), self.0.z());
        Self(EucVecf4::new(self.0.w(), vec.x(), vec.y(), self.0.x()) / self.det())
    }
}

impl Mul<EucVecf2> for Matf2 {
    type Output = EucVecf2;

    #[inline(always)]
    fn mul(self, rhs: EucVecf2) -> Self::Output {
        unsafe {
            let mul = vmulq_f32(self.0.0, vcombine_f32(rhs.0, rhs.0));
            let v1 = vtrn2q_f32(mul, mul);
            let v2 = vtrn1q_f32(mul, mul);

            let res = vaddq_f32(v1, v2);
            EucVecf2(vget_low_f32(vuzp2q_f32(res, res)))
        }
    }
}

impl Mul for Matf2 {
    type Output = Matf2;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = vtrn1q_f32(self.0.0, self.0.0);
            let v2 = vget_low_f32(rhs.0.0);
            let v2 = vcombine_f32(v2, v2);
            let m1 = vmulq_f32(v1, v2);

            let v3 = vtrn2q_f32(self.0.0, self.0.0);
            let v4 = vget_high_f32(rhs.0.0);
            let v4 = vcombine_f32(v4, v4);
            let m2 = vmulq_f32(v3, v4);

            Self(EucVecf4(vaddq_f32(m1, m2)))
        }
    }
}

impl Neg for Matf2 {
    type Output = Matf2;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}