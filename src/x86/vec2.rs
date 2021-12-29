use std::{intrinsics::transmute, ops::{Add, Sub, Mul, Div}};
use_arch_x86!(_mm_set_ps);

impl_vecf!(
    EucVecf2, 
    |x: Self| _mm_set_ps(x.x, x.y, 0., 0.),
    |x: __m128| {
        let cast : (f32, f32, f32, f32) = transmute(x);
        Self::new(cast.0, cast.1)
    }
);

impl EucVecf2 {
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    } 
}