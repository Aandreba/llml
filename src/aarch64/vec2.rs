use std::{arch::aarch64::{float32x2_t, vadd_f32, vsub_f32, vmul_f32, vmul_n_f32, vdiv_f32, vaddv_f32}, ops::{Add, Sub, Mul, Div}};
use crate::EucVecf2;

// FLOAT
map_to_trait!(EucVecf2, Add, add, |x: Self, y: Self| vadd_f32(x.casted(), y.casted()).into());
map_to_trait!(EucVecf2, Sub, sub, |x: Self, y: Self| vsub_f32(x.casted(), y.casted()).into());
map_to_trait!(EucVecf2, Mul, mul, |x: Self, y: Self| vmul_f32(x.casted(), y.casted()).into());
map_to_trait!(EucVecf2, Div, div, |x: Self, y: Self| vdiv_f32(x.casted(), y.casted()).into());

map_to_trait!(EucVecf2, Add, f32, add, EucVecf2, |x: Self, y: f32| vadd_f32(x.casted(), *(&[y,y] as *const [f32;2] as *const float32x2_t)).into());
map_to_trait!(f32, Add, EucVecf2, add, EucVecf2, |x: Self, y: EucVecf2| vadd_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), y.casted()).into());

map_to_trait!(EucVecf2, Sub, f32, sub, EucVecf2, |x: Self, y: f32| vsub_f32(x.casted(), *(&[y,y] as *const [f32;2] as *const float32x2_t)).into());
map_to_trait!(f32, Sub, EucVecf2, sub, EucVecf2, |x: Self, y: EucVecf2| vsub_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), y.casted()).into());

map_to_trait!(EucVecf2, Mul, f32, mul, EucVecf2, |x: Self, y: f32| vmul_n_f32(x.casted(), y).into());
map_to_trait!(f32, Mul, EucVecf2, mul, EucVecf2, |x: Self, y: EucVecf2| vmul_n_f32(y.casted(), x).into());

impl EucVecf2 {
    #[inline(always)]
    pub(crate) unsafe fn casted (self) -> float32x2_t {
        *(&self as *const EucVecf2 as *const float32x2_t)
    }

    #[inline(always)]
    pub fn sum (self) -> f32 {
        unsafe { vaddv_f32(self.casted()) }
    }
    
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        unsafe { vaddv_f32(vmul_f32(self.casted(), rhs.casted())) }
    }
}

impl From<float32x2_t> for EucVecf2 {
    fn from (x: float32x2_t) -> Self {
        unsafe { *(&x as *const float32x2_t as *const EucVecf2) }
    }
}

impl Into<float32x2_t> for EucVecf2 {
    fn into(self) -> float32x2_t {
        unsafe { self.casted() }
    }
}