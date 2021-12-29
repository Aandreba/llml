use std::{arch::aarch64::{float32x2_t, vadd_f32, vsub_f32, vmul_f32, vmul_n_f32, vdiv_f32, vaddv_f32}, ops::{Add, Sub, Mul, Div}};
use core::mem::transmute;
use crate::EucVecf2;

// FLOAT
map_to_trait!(EucVecf2, Add, add, |x: Self, y: Self| transmute(vadd_f32(transmute(x), transmute(y))));
map_to_trait!(EucVecf2, Sub, sub, |x: Self, y: Self| transmute(vsub_f32(transmute(x), transmute(y))));
map_to_trait!(EucVecf2, Mul, mul, |x: Self, y: Self| transmute(vmul_f32(transmute(x), transmute(y))));
map_to_trait!(EucVecf2, Div, div, |x: Self, y: Self| transmute(vdiv_f32(transmute(x), transmute(y))));

map_to_trait!(EucVecf2, Add, f32, add, EucVecf2, |x: Self, y: f32| transmute(vadd_f32(transmute(x), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Add, EucVecf2, add, EucVecf2, |x: Self, y: EucVecf2| transmute(vadd_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), transmute(y))));

map_to_trait!(EucVecf2, Sub, f32, sub, EucVecf2, |x: Self, y: f32| transmute(vsub_f32(transmute(x), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Sub, EucVecf2, sub, EucVecf2, |x: Self, y: EucVecf2| transmute(vsub_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), transmute(y))));

map_to_trait!(EucVecf2, Mul, f32, mul, EucVecf2, |x: Self, y: f32| transmute(vmul_n_f32(transmute(x), y)));
map_to_trait!(f32, Mul, EucVecf2, mul, EucVecf2, |x: Self, y: EucVecf2| transmute(vmul_n_f32(transmute(y), x)));

map_to_trait!(EucVecf2, Div, f32, div, EucVecf2, |x: Self, y: f32| transmute(vdiv_f32(transmute(x), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Div, EucVecf2, div, EucVecf2, |x: Self, y: EucVecf2| transmute(vdiv_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), transmute(y))));

impl EucVecf2 {
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        unsafe { vaddv_f32(vmul_f32(transmute(self), transmute(rhs))) }
    }
}

impl From<float32x2_t> for EucVecf2 {
    fn from (x: float32x2_t) -> Self {
        unsafe { transmute(x) }
    }
}

impl Into<float32x2_t> for EucVecf2 {
    fn into(self) -> float32x2_t {
        unsafe { transmute(self) }
    }
}