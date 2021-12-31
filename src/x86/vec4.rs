use std::{ops::{Add, Sub, Mul, Div}, intrinsics::transmute};
use_arch_x86!(_mm_shuffle_ps, _mm_movehl_ps, _mm_cvtss_f32, _MM_SHUFFLE);

impl_vecf!(
    EucVecf4, 
    |x: Self| transmute(x),
    |x: __m128| transmute(x)
);

impl EucVecf4 {
    #[inline(always)]
    pub fn sum (self) -> f32 {
        unsafe {
            let mul = self.casted();

            let shuf = _mm_shuffle_ps(mul, mul, _MM_SHUFFLE(2, 3, 0, 1));
            let sums = _mm_add_ps(mul, shuf);
            
            let shuf = _mm_movehl_ps(shuf, sums);
            let sums = _mm_add_ps(sums, shuf);
            
            return _mm_cvtss_f32(sums);
        }
    }

    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        unsafe {
            let mul = _mm_mul_ps(self.casted(), rhs.casted());

            let shuf = _mm_shuffle_ps(mul, mul, _MM_SHUFFLE(2, 3, 0, 1));
            let sums = _mm_add_ps(mul, shuf);
            
            let shuf = _mm_movehl_ps(shuf, sums);
            let sums = _mm_add_ps(sums, shuf);
            
            return _mm_cvtss_f32(sums);
        }
    }
}