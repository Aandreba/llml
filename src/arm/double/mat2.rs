arm_use!();
use std::{ops::{Add, Sub, Mul, Div, Neg}};
use crate::{arm::{EucVecd4, EucVecd2, Matf2}, others::Zero};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Matd2 (pub(crate) EucVecd4);
impl_mat2!(Matd2, f64);

impl Matd2 {
    #[inline(always)]
    pub fn new (a: [f64;4]) -> Self {
        Self(EucVecd4::new(a))
    }

    #[inline(always)]
    pub fn x (&self) -> EucVecd2 {
        self.0.0
    }

    #[inline(always)]
    pub fn xx (&self) -> f64 {
        self.0.x()
    }

    #[inline(always)]
    pub fn xy (&self) -> f64 {
        self.0.y()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVecd2 {
        self.0.1
    }

    #[inline(always)]
    pub fn yx (&self) -> f64 {
        self.0.z()
    }

    #[inline(always)]
    pub fn yy (&self) -> f64 {
        self.0.w()
    }

    #[inline(always)]
    pub fn tr (self) -> f64 {
        self.0.x()  +self.0.w()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        let m1 : EucVecd2 = self.0.0 * EucVecd2::new([self.0.1.y(), self.0.1.x()]);
        m1.x() - m1.y()
    }

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        let vec = -EucVecd2::new([self.0.y(), self.0.z()]);
        Some(Self(EucVecd4::new([self.0.w(), vec.x(), vec.y(), self.0.x()]) / det))
    }

    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        let vec = -EucVecd2::new([self.0.y(), self.0.z()]);
        Self(EucVecd4::new([self.0.w(), vec.x(), vec.y(), self.0.x()]) / self.det())
    }
}

impl Mul<EucVecd2> for Matd2 {
    type Output = EucVecd2;

    #[inline(always)]
    fn mul(self, rhs: EucVecd2) -> Self::Output {
        let mul = self.0 * EucVecd4(rhs, rhs);
        let v1 = EucVecd2::new([mul.y(), mul.w()]); // odd
        let v2 = EucVecd2::new([mul.x(), mul.z()]); // even
        v1 + v2
    }
}


impl Mul for Matd2 {
    type Output = Matd2;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 : EucVecd4 = EucVecd4(EucVecd2(vdupq_n_f64(self.0.x())), EucVecd2(vdupq_n_f64(self.0.z())));
            let v2 : EucVecd4 = EucVecd4(rhs.0.0, rhs.0.0);
            let m1 = v1 * v2;

            let v3 : EucVecd4 = EucVecd4(EucVecd2(vdupq_n_f64(self.0.y())), EucVecd2(vdupq_n_f64(self.0.w())));
            let v4 : EucVecd4 = EucVecd4(rhs.0.1, rhs.0.1);
            let m2 = v3 * v4;

            Self(m1 + m2)
        }
    }
}

impl Neg for Matd2 {
    type Output = Matd2;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Into<[f64;4]> for Matd2 {
    #[inline(always)]
    fn into(self) -> [f64;4] {
        self.0.into()
    }
}

impl Into<Matf2> for Matd2 {
    #[inline(always)]
    fn into(self) -> Matf2 {
        Matf2(self.0.into())
    }
}