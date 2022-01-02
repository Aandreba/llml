use std::{ops::Add, arch::x86_64::{__m256, _mm256_add_ps}};
use crate::{mat::Matf3, vec::EucVecf3};

impl Add for Matf3 {
    type Output = Matf3;

    #[inline(always)]
    fn add (self, rhs: Self) -> Self::Output {
        unsafe {
            Self::from(_mm256_add_ps(self.into_vec(), rhs.into_vec()), self.z.z + rhs.z.z)
        }
    }
}

impl Matf3 {
    #[inline(always)]
    pub(crate) unsafe fn into_vec (self) -> __m256 {
        *(&self as *const Self as *const __m256)
    }

    #[inline(always)]
    pub(crate) unsafe fn from (alpha: __m256, beta: f32) -> Self {
        let alpha = &alpha as *const __m256 as *const EucVecf3;
        let gamma = alpha.add(2) as *const f32;

        Self::new(
            *alpha, 
            *alpha.add(1), 
            EucVecf3::new(*gamma, *gamma.add(1), beta)
        )
    }
}