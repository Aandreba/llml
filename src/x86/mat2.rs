use std::{intrinsics::transmute, ops::{Add, Sub, Mul}};
use crate::Matf2;

use_arch_x86!(__m128, _mm_add_ps, _mm_sub_ps, _mm_mul_ps, _mm_div_ps, _mm_set_ps);

map_to_trait!(Matf2, Add, add, |x: Self, y: Self| Self::from_vec(_mm_add_ps(x.as_vec(), y.as_vec())));
map_to_trait!(Matf2, Sub, sub, |x: Self, y: Self| Self::from_vec(_mm_sub_ps(x.as_vec(), y.as_vec())));

impl Mul for Matf2 {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let lhs = self.as_vec();
            let rhs1 = _mm_set_ps(rhs.y.y, rhs.x.y, rhs.y.x, rhs.x.x);
            let rhs2 = _mm_set_ps(rhs.y.x, rhs.x.x, rhs.y.y, rhs.x.y);

            let m1 = &_mm_mul_ps(lhs, rhs1) as *const __m128 as *const f32;
            let m2 = &_mm_mul_ps(lhs, rhs2) as *const __m128 as *const f32;
            
            Self::of_values(
                *m1 + *m1.add(1), *m2 + *m2.add(1), 
                *m2.add(2) + *m2.add(3), *m1.add(2) + *m1.add(3)
            )
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
        unsafe {Self::from_vec(_mm_div_ps(self.as_vec(), rhs.as_vec())) }
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