use std::{intrinsics::transmute, ops::{Add, Sub, Mul, Div}};
use_arch_x86!(_mm_set_ps);

impl_vecf!(
    EucVecf3, 
    |x: Self| _mm_set_ps(x.x, x.y, x.z, 0.),
    |x: __m128| {
        let cast : (f32, f32, f32, f32) = transmute(x);
        Self::new(cast.0, cast.1, cast.2)
    }
);

impl EucVecf3 {
    pub fn sum (self) -> f32 {
        todo!()
    }
    
    pub fn dot (self, rhs: Self) -> f32 {
        (self * rhs).sum()
    } 
}