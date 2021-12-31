use std::{arch::x86_64::{__m256, _mm256_add_ps}, ops::Add};
use crate::{mat::{Matf3}, vec::EucVecf3};

map_to_trait!(Matf3, Add, add, |x: Self, y: Self| {
    let (v1, s1) = x.casted();
    let (v2, s2) = y.casted();

    // TODO: APARENTLY, THIS IS SUPERSLOW (EVEN SLOWER THAN NAIVE)
    Self::unsafe_from(_mm256_add_ps(v1, v2), s1 + s2)
});

impl Matf3 {
    #[inline(always)]
    pub unsafe fn casted (self) -> (__m256, f32) {
        let ptr = &self as *const Self;
        let vec = *(ptr as *const __m256);
        let scal = *(ptr as *const f32).add(8);

        (vec, scal)
    }

    #[inline(always)]
    pub unsafe fn unsafe_from (vec: __m256, scal: f32) -> Self {
        let vec = &vec as *const __m256 as *const EucVecf3;

        let x = *vec;
        let y = *vec.add(1);
        
        let vec = vec.add(2) as *const f32;
        let z = EucVecf3::new(*vec, *vec.add(1), scal);

        Self::new(x, y, z)
    }
}