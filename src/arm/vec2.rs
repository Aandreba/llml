arm_use!();
use core::mem::transmute;
use std::ptr::addr_of;
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};

wrap!(
    EucVecf2, float32x2_t,
    EucVecd2, float64x2_t
);

impl_vec2!(EucVecf2, f32, u32);
impl_vec2!(EucVecd2, f64, q, u64);

impl Into<EucVecf2> for EucVecd2 {
    #[inline(always)]
    fn into(self) -> EucVecf2 {
        unsafe { EucVecf2(vcvt_f32_f64(self.0)) }
    }
}

impl Into<EucVecd2> for EucVecf2 {
    #[inline(always)]
    fn into(self) -> EucVecd2 {
        unsafe { EucVecd2(vcvt_f64_f32(self.0)) }
    }
}