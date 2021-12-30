use std::{arch::x86_64::__m256, alloc::{alloc, Layout}, intrinsics::transmute};
use crate::mat::{Matf3};

impl Matf3 {
    const LAYOUT : Layout = Layout::new::<[f32;9]>();

    #[inline(always)]
    pub unsafe fn casted (self) -> (__m256, f32) {
        let ptr = &self as *const Self;
        let vec = *(ptr as *const __m256);
        let scal = *(ptr as *const f32).add(8);

        (vec, scal)
    }

    #[inline(always)]
    pub unsafe fn unsafe_from (vec: __m256, scal: f32) -> Self {
        todo!()
    }
}