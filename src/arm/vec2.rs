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