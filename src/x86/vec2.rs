use std::{ops::{Add, Sub, Mul, Div}};
use_arch_x86!(_mm_set_ps);

impl_vecf!(
    EucVecf2, 
    |x: Self| _mm_set_ps(0., 0., x.y, x.x),
    |x: __m128| {
        let ptr = &x as *const __m128 as *const f32;
        Self::new(*ptr, *ptr.add(1))
    }
);

impl EucVecf2 {
    // TO BENCH
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    } 
}