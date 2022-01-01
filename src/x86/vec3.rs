use std::{intrinsics::transmute, ops::{Add, Sub, Mul, Div}};
use_arch_x86!(_mm_set_ps, _mm_shuffle_ps, _mm_movehl_ps, _mm_cvtss_f32, _MM_SHUFFLE);

impl_vecf!(
    EucVecf3, 
    |x: Self| _mm_set_ps(0., x.z, x.y, x.x),
    |x: __m128| {
        let ptr = &x as *const __m128 as *const f32;
        Self::new(*ptr, *ptr.add(1), *ptr.add(2))
    }
);

impl EucVecf3 {
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
            Self::raw_dot(self.casted(), rhs.casted())
        }
    }

    #[inline(always)]
    pub fn cross (self, rhs: Self) -> Self {
        unsafe {
            let v1 = _mm_set_ps(0., self.x, rhs.x, self.y);
            let v2 = _mm_set_ps(0., rhs.y, self.z, rhs.z);
            let m1 = _mm_mul_ps(v1, v2);
            
            let v3 = _mm_set_ps(0., self.y, rhs.z, self.z);
            let v4 = _mm_set_ps(0., rhs.x, self.x, rhs.y);
            let m2 = _mm_mul_ps(v3, v4);

            Self::unsafe_from(_mm_sub_ps(m1, m2))
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn raw_dot (alpha: __m128, beta: __m128) -> f32 {
        let mul = _mm_mul_ps(alpha, beta);

        let shuf = _mm_shuffle_ps(mul, mul, _MM_SHUFFLE(2, 3, 0, 1));
        let sums = _mm_add_ps(mul, shuf);
        
        let shuf = _mm_movehl_ps(shuf, sums);
        let sums = _mm_add_ps(sums, shuf);
        
        return _mm_cvtss_f32(sums);
    }
}