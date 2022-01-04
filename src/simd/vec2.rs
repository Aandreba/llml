use std::{simd::{Simd, SimdElement}};
use crate::vec::EucVec2;

simd_map!(EucVec2);

impl<T: SimdElement> EucVec2<T> {
    #[inline(always)]
    pub(crate) unsafe fn into_simd (self) -> Simd<T,2> {
        *(&self as *const Self as *const Simd<T,2>)
    } 

    #[inline(always)]
    pub(crate) unsafe fn from_simd (x: Simd<T,2>) -> Self {
        *(&x as *const Simd<T,2> as *const Self)
    }
}

impl<T: SimdElement> Into<Simd<T,2>> for EucVec2<T> {
    fn into(self) -> Simd<T,2> {
        unsafe { self.into_simd() }
    }
}

impl <T: SimdElement> From<Simd<T,2>> for EucVec2<T> {
    fn from(x: Simd<T,2>) -> Self {
        unsafe { Self::from_simd(x) }
    }
}