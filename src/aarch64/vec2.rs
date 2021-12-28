use std::{arch::aarch64::{float32x2_t, vadd_f32, vsub_f32, vmul_f32, vmul_n_f32, vdiv_f32, vaddv_f32}, ops::{Add, Sub, Mul, Div}};
use crate::EucVecf2;

// FLOAT
map_to_trait!(EucVecf2, Add, add, |x: Self, y: Self| Self::unsafe_from(vadd_f32(x.casted(), y.casted())));
map_to_trait!(EucVecf2, Sub, sub, |x: Self, y: Self| Self::unsafe_from(vsub_f32(x.casted(), y.casted())));
map_to_trait!(EucVecf2, Mul, mul, |x: Self, y: Self| Self::unsafe_from(vmul_f32(x.casted(), y.casted())));
map_to_trait!(EucVecf2, Div, div, |x: Self, y: Self| Self::unsafe_from(vdiv_f32(x.casted(), y.casted())));

map_to_trait!(EucVecf2, Add, f32, add, EucVecf2, |x: Self, y: f32| Self::unsafe_from(vadd_f32(x.casted(), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Add, EucVecf2, add, EucVecf2, |x: Self, y: EucVecf2| EucVecf2::unsafe_from(vadd_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), y.casted())));

map_to_trait!(EucVecf2, Sub, f32, sub, EucVecf2, |x: Self, y: f32| Self::unsafe_from(vsub_f32(x.casted(), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Sub, EucVecf2, sub, EucVecf2, |x: Self, y: EucVecf2| EucVecf2::unsafe_from(vsub_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), y.casted())));

map_to_trait!(EucVecf2, Mul, f32, mul, EucVecf2, |x: Self, y: f32| Self::unsafe_from(vmul_n_f32(x.casted(), y)));
map_to_trait!(f32, Mul, EucVecf2, mul, EucVecf2, |x: Self, y: EucVecf2| EucVecf2::unsafe_from(vmul_n_f32(y.casted(), x)));

map_to_trait!(EucVecf2, Div, f32, div, EucVecf2, |x: Self, y: f32| Self::unsafe_from(vdiv_f32(x.casted(), *(&[y,y] as *const [f32;2] as *const float32x2_t))));
map_to_trait!(f32, Div, EucVecf2, div, EucVecf2, |x: Self, y: EucVecf2| EucVecf2::unsafe_from(vdiv_f32(*(&[x,x] as *const [f32;2] as *const float32x2_t), y.casted())));

impl EucVecf2 {
    #[inline(always)]
    pub(crate) unsafe fn unsafe_from (x: float32x2_t) -> Self {
        *(&x as *const float32x2_t as *const EucVecf2)
    }

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
        unsafe { Self::unsafe_from(x) }
    }
}

impl Into<float32x2_t> for EucVecf2 {
    fn into(self) -> float32x2_t {
        unsafe { self.casted() }
    }
}