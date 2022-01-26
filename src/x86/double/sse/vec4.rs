x86_use!();
use crate::{x86::vec4::EucVecf4, vec::EucVecd2};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::mem::transmute;

#[repr(C, align(32))]
pub struct EucVecd4 (pub(crate) EucVecd2, pub(crate) EucVecd2);
impl_arith_x2!(EucVecd4);

impl EucVecd4 {
    #[inline(always)]
    pub fn new (a: [f64;4]) -> Self {
        Self(EucVecd2::new([a[0], a[1]]), EucVecd2::new([a[2], a[3]]))
    }

    #[inline(always)]
    pub fn from_scal (x: f64) -> Self {
        Self(EucVecd2::from_scal(x), EucVecd2::from_scal(x))
    }

    #[inline(always)]
    pub fn x (&self) -> f64 {
        self.0.x()
    }

    #[inline(always)]
    pub fn y (&self) -> f64 {
        self.0.y()
    }

    #[inline(always)]
    pub fn z (&self) -> f64 {
        self.1.x()
    }

    #[inline(always)]
    pub fn w (&self) -> f64 {
        self.1.y()
    }

    #[inline(always)]
    pub fn sum (self) -> f64 {
        self.0.sum() + self.1.sum()
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f64 {
        (self * rhs).sum()
    }

    #[inline(always)]
    #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
    pub fn norm2 (self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm (self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn unit (self) -> Self {
        self / self.norm()
    }

    #[inline(always)]
    pub fn abs (self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    #[inline(always)]
    pub fn sqrt (self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt())
    }

    #[inline(always)]
    pub fn sqrt_fast (self) -> Self {
        Self(self.0.sqrt_fast(), self.1.sqrt_fast())
    }
}

impl Into<[f64;4]> for EucVecd4 {
    #[inline(always)]
    fn into (self) -> [f64;4] {
        unsafe { transmute(self) }
    }
}

impl Into<EucVecf4> for EucVecd4 {
    fn into (self) -> EucVecf4 {
        unsafe {
            EucVecf4(_mm_shuffle_ps(_mm_cvtpd_ps(self.0.0),  _mm_cvtpd_ps(self.1.0), _MM_SHUFFLE(1, 0, 1, 0)))
        }
    }
}