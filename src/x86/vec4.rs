use std::{ops::{Add, Sub, Mul, Div}, intrinsics::transmute};
use_arch_x86!(_mm_dp_ps, _mm_extract_ps);

impl_vecf!(
    EucVecf4, 
    |x: Self| transmute(x),
    |x: __m128| transmute(x)
);

#[cfg(feature = "sse41")]
impl EucVecf4 {
    #[inline(always)]
    pub fn sum (self) -> f32 {
        unsafe {
            let dot = _mm_dp_ps::<0xf1>(self.casted(), transmute([1f32, 1., 1., 1.]));
            transmute(_mm_extract_ps::<0>(dot))
        }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        unsafe {
            let dot = _mm_dp_ps::<0xf1>(self.casted(), rhs.casted());
            transmute(_mm_extract_ps::<0>(dot))
        }
    }
}