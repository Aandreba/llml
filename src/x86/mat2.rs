use std::{intrinsics::transmute, ops::{Add, Sub, Mul, Div}};
use crate::{mat::Matf2, others::Complxf, vec::EucVecf2};

use_arch_x86!(__m128, _mm_set1_ps, _mm_add_ps, _mm_sub_ps, _mm_mul_ps, _mm_div_ps, _mm_set_ps);

map_to_trait!(Matf2, Add, add, |x: Self, y: Self| Self::from_vec(_mm_add_ps(x.as_vec(), y.as_vec())));
map_to_trait!(Matf2, Add, f32, add, Matf2, |x: Self, y: f32| Self::from_vec(_mm_add_ps(x.as_vec(), _mm_set1_ps(y))));
map_to_trait!(f32, Add, Matf2, add, Matf2, |x: Self, y: Matf2| Matf2::from_vec(_mm_add_ps(_mm_set1_ps(x), y.as_vec())));

map_to_trait!(Matf2, Sub, sub, |x: Self, y: Self| Self::from_vec(_mm_sub_ps(x.as_vec(), y.as_vec())));
map_to_trait!(Matf2, Sub, f32, sub, Matf2, |x: Self, y: f32| Self::from_vec(_mm_sub_ps(x.as_vec(), _mm_set1_ps(y))));
map_to_trait!(f32, Sub, Matf2, sub, Matf2, |x: Self, y: Matf2| Matf2::from_vec(_mm_sub_ps(_mm_set1_ps(x), y.as_vec())));

map_to_trait!(Matf2, Mul, f32, mul, Matf2, |x: Self, y: f32| Self::from_vec(_mm_mul_ps(x.as_vec(), _mm_set1_ps(y))));
map_to_trait!(f32, Mul, Matf2, mul, Matf2, |x: Self, y: Matf2| Matf2::from_vec(_mm_mul_ps(_mm_set1_ps(x), y.as_vec())));

map_to_trait!(Matf2, Div, f32, div, Matf2, |x: Self, y: f32| Self::from_vec(_mm_div_ps(x.as_vec(), _mm_set1_ps(y))));
map_to_trait!(f32, Div, Matf2, div, Matf2, |x: Self, y: Matf2| Matf2::from_vec(_mm_div_ps(_mm_set1_ps(x), y.as_vec())));

impl Mul for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = _mm_set_ps(self.y.x, self.y.x, self.x.x, self.x.x);
            let v2 = _mm_set_ps(rhs.x.y, rhs.x.x, rhs.x.y, rhs.x.x);
            let m1 = _mm_mul_ps(v1, v2);

            let v3 = _mm_set_ps(self.y.y, self.y.y, self.x.y, self.x.y);
            let v4 = _mm_set_ps(rhs.y.y, rhs.y.x, rhs.y.y, rhs.y.x);
            let m2 = _mm_mul_ps(v3, v4);

            Self::from_vec(_mm_add_ps(m1, m2))
        }
    }
}

impl Mul<EucVecf2> for Matf2 {
    type Output = EucVecf2;

    #[inline(always)]
    fn mul (self, rhs: EucVecf2) -> Self::Output {
        unsafe {
            let v1 = self.as_vec();
            let v2 = _mm_set_ps(rhs.y, rhs.x, rhs.y, rhs.x);
            let m1 = &_mm_mul_ps(v1, v2) as *const __m128 as *const f32;

            let v3 = _mm_set_ps(0., 0., *m1.add(2), *m1);
            let v4 = _mm_set_ps(0., 0., *m1.add(3), *m1.add(1));
            EucVecf2::unsafe_from(_mm_add_ps(v3, v4))
        }
    }
}

impl Matf2 {
    #[inline(always)]
    pub fn scal_mul (self, rhs: Self) -> Self {
        unsafe { Self::from_vec(_mm_mul_ps(self.as_vec(), rhs.as_vec())) }
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        unsafe { Self::from_vec(_mm_div_ps(self.as_vec(), rhs.as_vec())) }
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        unsafe {
            let rhs = _mm_set_ps(0., 0., self.y.x, self.y.y);
            let mul = &_mm_mul_ps(self.as_vec(), rhs) as *const __m128 as *const f32;
            *mul - *mul.add(1)
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn as_vec (self) -> __m128 {
        transmute(self)
    }

    #[inline(always)]
    pub(crate) unsafe fn from_vec (x: __m128) -> Self {
        transmute(x)
    }
}