x86_use!();
use crate::{x86::_mm256_movehdup_pd, mat::Mat2f};
use std::{ops::{Add, Sub, Mul, Div, Neg}, ptr::addr_of};
use crate::{others::Zero, vec::EucVec2d, x86::_mm256_combine_pd};
use super::{vec4::EucVec4d};
use crate::x86::{_mm256_low_pd, _mm256_high_pd};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Mat2d (pub(crate) EucVec4d);

impl Mat2d {
    #[inline]
    pub fn new (a: [f64;4]) -> Self {
        Self(a.into())
    }

    #[inline]
    pub fn of_rot (a: f64) -> Self {
        let (sin, cos) = a.sin_cos();
        Self::new([cos, -sin, sin, cos])
    }

    #[inline(always)]
    pub fn x (&self) -> EucVec2d {
        unsafe { EucVec2d(_mm256_high_pd(self.0.0)) }
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
    pub fn y (&self) -> EucVec2d {
        unsafe { EucVec2d(_mm256_low_pd(self.0.0)) }
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
    pub fn scal_mul (self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    #[inline(always)]
    pub fn tr (self) -> f64 {
        self.0.x() + self.0.w()
    }

    #[inline(always)]
    pub fn det (self) -> f64 {
        unsafe {
            let v2 = _mm256_extractf128_pd(self.0.0, _MM_SHUFFLE(0, 0, 2, 3));
            let m1 = self.x() * EucVec2d(v2);
            //let m1 = EucVec2d(_mm_mul_pd(self.0.0, v2));
            m1.x() - m1.y()
        }
    }

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        unsafe { Some(self._inv(det)) }
    }

    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        self._inv(self.det())
    }

    #[inline(always)]
    unsafe fn _inv (self, det: f64) -> Self {
        let beta : *const f64 = addr_of!(self.0.0).cast();
        let beta = _mm_load_pd(beta.add(1));
        let neg = EucVec2d(_mm_sub_pd(_mm_setzero_pd(), beta));
        Self(EucVec4d::new([self.0.w(), neg.x(), neg.y(), self.0.x()]) / det)
    }
}

trait_map!(
    Mat2d, f64,
    Add, add,
    Sub, sub
);

trait_map_scal!(
    Mat2d, f64,
    Mul, mul,
    Div, div
);

impl Neg for Mat2d {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<EucVec2d> for Mat2d {
    type Output = EucVec2d;

    #[inline(always)]
    fn mul (self, rhs: EucVec2d) -> Self::Output {
        unsafe {
            let v1 = self.0.0;
            let v2 = _mm256_combine_pd(rhs.0, rhs.0);

            let m1 = _mm256_mul_pd(v1, v2);
            let v1 = _mm256_movedup_pd(m1);

            let res = _mm256_add_pd(m1, v1);
            EucVec2d(_mm256_extractf128_pd(res, _MM_SHUFFLE(0, 0, 3, 1)))
        }
    }
}

impl Mul for Mat2d {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = _mm256_movedup_pd(self.0.0);
            let v3 = _mm256_movehdup_pd(self.0.0);
            
            let v2 = _mm256_shuffle_pd(rhs.0.0, rhs.0.0, _MM_SHUFFLE(1, 0, 1, 0));
            let v4 = _mm256_shuffle_pd(rhs.0.0, rhs.0.0, _MM_SHUFFLE(3, 2, 3, 2));

            let m1 = _mm256_mul_pd(v1, v2);
            let m2 = _mm256_mul_pd(v3, v4);
            Self(EucVec4d(_mm256_add_pd(m1, m2)))
        }
    }
}

impl Into<[f64;4]> for Mat2d {
    #[inline(always)]
    fn into(self) -> [f64;4] {
        self.0.into()
    }
}

impl Into<Mat2f> for Mat2d {
    #[inline(always)]
    fn into(self) -> Mat2f {
        Mat2f(self.0.into())
    }
}