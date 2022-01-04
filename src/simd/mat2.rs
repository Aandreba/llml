use std::simd::{SimdElement, Simd};
use crate::mat::Mat2;

simd_mat_map!(Mat2);

impl<T: SimdElement> Mat2<T> {
    #[inline(always)]
    pub(crate) unsafe fn into_simd (self) -> Simd<T,4> {
        *(&self as *const Self as *const Simd<T,4>)
    } 

    #[inline(always)]
    pub(crate) unsafe fn from_simd (x: Simd<T,4>) -> Self {
        *(&x as *const Simd<T,4> as *const Self)
    }
}

impl<T: SimdElement> Into<Simd<T,4>> for Mat2<T> {
    fn into(self) -> Simd<T,4> {
        unsafe { self.into_simd() }
    }
}

impl <T: SimdElement> From<Simd<T,4>> for Mat2<T> {
    fn from(x: Simd<T,4>) -> Self {
        unsafe { Self::from_simd(x) }
    }
}