use std::{intrinsics::transmute, ops::{Add, Sub, Mul, Div}};
use_arch_x86!(_mm_set_ps);

impl_vecf!(
    EucVecf2, 
    |x: Self| _mm_set_ps(x.x, x.y, 0., 0.),
    |x: __m128| {
        let ptr = &x as *const __m128 as *const f32;
        Self::new(*(ptr.add(2)), *(ptr.add(3)))
    }
);

impl EucVecf2 {
    // TO BENCH
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    } 
}