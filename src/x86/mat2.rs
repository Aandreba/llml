x86_use!();

use cfg_if::cfg_if;
use std::{ops::{Add, Sub, Mul, Div, Neg}};
use crate::{x86::{_mm_low_ps, _mm_high_ps}, others::Zero};
use super::{vec4::EucVecf4, vec2::EucVecf2, Matd2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Matf2 (pub(crate) EucVecf4);

impl Matf2 {
    #[inline]
    pub fn new (a: [f32;4]) -> Self {
        Self(a.into())
    }

    #[inline]
    pub fn of_rot (a: f32) -> Self {
        let (sin, cos) = a.sin_cos();
        Self::new([cos, -sin, sin, cos])
    }

    #[inline(always)]
    pub fn x (&self) -> EucVecf2 {
        unsafe { EucVecf2(_mm_low_ps(self.0.0)) }
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
        unsafe { EucVecf2(_mm_high_ps(self.0.0)) }
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
    pub fn scal_mul (self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    #[inline(always)]
    pub fn tr (self) -> f32 {
        self.0.x() + self.0.w()
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        unsafe {
            let v2 = _mm_shuffle_ps(self.0.0, _mm_setzero_ps(), _MM_SHUFFLE(0, 0, 2, 3));
            let m1 = EucVecf2(_mm_mul_ps(self.0.0, v2));

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
    unsafe fn _inv (self, det: f32) -> Self {
        let neg = EucVecf4(_mm_sub_ps(_mm_setzero_ps(), _mm_shuffle_ps(self.0.0, self.0.0, _MM_SHUFFLE(0, 0, 2, 1))));
        Self(EucVecf4::new([self.0.w(), neg.x(), neg.y(), self.0.x()]) / det)
    }
}

trait_map!(
    Matf2, f32,
    Add, add,
    Sub, sub
);

trait_map_scal!(
    Matf2, f32,
    Mul, mul,
    Div, div
);

impl Neg for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<EucVecf2> for Matf2 {
    type Output = EucVecf2;

    #[inline(always)]
    fn mul (self, rhs: EucVecf2) -> Self::Output {
        unsafe {
            let v1 = self.0.0;
            let v2 = _mm_shuffle_ps(rhs.0, rhs.0, _MM_SHUFFLE(1, 0, 1, 0));
            let m1 = _mm_mul_ps(v1, v2);
            
            let v1 : __m128;
            cfg_if! {
                if #[cfg(target_feature = "sse3")] {
                    v1 = _mm_moveldup_ps(m1)
                } else {
                    v1 = _mm_shuffle_ps(m1, m1, _MM_SHUFFLE(2, 2, 0, 0));
                }
            }

            EucVecf2(_mm_shuffle_ps(_mm_add_ps(m1, v1), _mm_setzero_ps(), _MM_SHUFFLE(0, 0, 3, 1)))
        }
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

impl Into<Matd2> for Matf2 {
    #[inline(always)]
    fn into(self) -> Matd2 {
        Matd2(self.0.into())
    }
}