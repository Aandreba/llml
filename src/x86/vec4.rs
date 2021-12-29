use_arch_x86!(__m128, _mm_add_ps, _mm_sub_ps, _mm_mul_ps, _mm_div_ps);

#[cfg(target_feature = "sse4.1")]
use_arch_x86!(_mm_dp_ps);

#[cfg(target_feature = "sse3")]
use_arch_x86!(_mm_hadd_ps);

use std::{ops::{Add, Sub, Mul, Div}, intrinsics::transmute};
use crate::EucVecf4;

map_to_trait!(EucVecf4, Add, add, |x: Self, y: Self| transmute(_mm_add_ps(transmute(x), transmute(y))));
map_to_trait!(EucVecf4, Sub, sub, |x: Self, y: Self| transmute(_mm_sub_ps(transmute(x), transmute(y))));
map_to_trait!(EucVecf4, Mul, mul, |x: Self, y: Self| transmute(_mm_mul_ps(transmute(x), transmute(y))));
map_to_trait!(EucVecf4, Div, div, |x: Self, y: Self| transmute(_mm_div_ps(transmute(x), transmute(y))));

map_to_trait!(EucVecf4, Add, f32, add, EucVecf4, |x: Self, y: f32| transmute(_mm_add_ps(transmute(x), transmute([y,y,y,y]))));
map_to_trait!(f32, Add, EucVecf4, add, EucVecf4, |x: Self, y: EucVecf4| transmute(_mm_add_ps(transmute([x,x,x,x]), transmute(y))));

map_to_trait!(EucVecf4, Sub, f32, sub, EucVecf4, |x: Self, y: f32| transmute(_mm_sub_ps(transmute(x), transmute([y,y,y,y]))));
map_to_trait!(f32, Sub, EucVecf4, sub, EucVecf4, |x: Self, y: EucVecf4| transmute(_mm_sub_ps(transmute([x,x,x,x]), transmute(y))));

map_to_trait!(EucVecf4, Mul, f32, mul, EucVecf4, |x: Self, y: f32| transmute(_mm_mul_ps(transmute(x), transmute([y,y,y,y]))));
map_to_trait!(f32, Mul, EucVecf4, mul, EucVecf4, |x: Self, y: EucVecf4| transmute(_mm_mul_ps(transmute([x,x,x,x]), transmute(y))));

map_to_trait!(EucVecf4, Div, f32, div, EucVecf4, |x: Self, y: f32| transmute(_mm_div_ps(transmute(x), transmute([y,y,y,y]))));
map_to_trait!(f32, Div, EucVecf4, div, EucVecf4, |x: Self, y: EucVecf4| transmute(_mm_div_ps(transmute([x,x,x,x]), transmute(y))));


impl EucVecf4 {
    /* SUM */
    #[cfg(target_feature = "sse3")]
    #[inline(always)]
    pub fn sum (self) -> f32 {
        todo!()
    }

    #[cfg(not(target_feature = "sse3"))]
    #[inline(always)]
    pub fn sum (self) -> f32 {
        self.x + self.y + self.z + self.w
    }

    /* DOT */
    // UNTESTED & TODO
    #[cfg(target_feature = "sse4.1")]
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        todo!()
    }

    #[cfg(not(target_feature = "sse4.1"))]
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    }
}