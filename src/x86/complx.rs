use std::{ops::{Add, Sub, Mul, Div}, intrinsics::transmute};
use crate::others::Complxf;

use_arch_x86!(__m128, _mm_set_ps, _mm_add_ps, _mm_sub_ps, _mm_mul_ps, _mm_div_ps);

map_to_trait!(Complxf, Add, add, |x: Self, y: Self| Self::unsafe_from(_mm_add_ps(x.casted(), y.casted())));
map_to_trait!(Complxf, Add, f32, add, Complxf, |x: Self, y: f32| Self::unsafe_from(_mm_add_ps(x.casted(), transmute((y,y,y,y)))));
map_to_trait!(f32, Add, Complxf, add, Complxf, |x: Self, y: Complxf| Complxf::unsafe_from(_mm_add_ps(transmute((x,x,x,x)), y.casted())));

map_to_trait!(Complxf, Sub, sub, |x: Self, y: Self| Self::unsafe_from(_mm_sub_ps(x.casted(), y.casted())));
map_to_trait!(Complxf, Sub, f32, sub, Complxf, |x: Self, y: f32| Self::unsafe_from(_mm_sub_ps(x.casted(), transmute((y,y,y,y)))));
map_to_trait!(f32, Sub, Complxf, sub, Complxf, |x: Self, y: Complxf| Complxf::unsafe_from(_mm_sub_ps(transmute((x,x,x,x)), y.casted())));

map_to_trait!(Complxf, Mul, mul, |x: Self, y: Self| {
    let v1 = _mm_set_ps(y.re, x.re, -x.im, x.re);
    let v2 = _mm_set_ps(x.im, y.im, y.im, y.re);
    let m1 = &_mm_mul_ps(v1, v2) as *const __m128 as *const f32;

    let v1 = _mm_set_ps(0., 0., *m1.add(2), *m1);
    let v2 = _mm_set_ps(0., 0., *m1.add(3), *m1.add(1));
    Self::unsafe_from(_mm_add_ps(v1, v2))
});
map_to_trait!(Complxf, Mul, f32, mul, Complxf, |x: Self, y: f32| Self::unsafe_from(_mm_mul_ps(x.casted(), transmute((y,y,y,y)))));
map_to_trait!(f32, Mul, Complxf, mul, Complxf, |x: Self, y: Complxf| Complxf::unsafe_from(_mm_mul_ps(transmute((x,x,x,x)), y.casted())));

map_to_trait!(Complxf, Div, div, |x: Self, y: Self| (x * y.conj()) / y.magn());
map_to_trait!(Complxf, Div, f32, div, Complxf, |x: Self, y: f32| Self::unsafe_from(_mm_div_ps(x.casted(), transmute((y,y,y,y)))));
map_to_trait!(f32, Div, Complxf, div, Complxf, |x: Self, y: Complxf| Complxf::unsafe_from(_mm_div_ps(transmute((x,x,x,x)), y.casted())));

impl Complxf {
    #[inline(always)]
    pub(crate) unsafe fn casted (self) -> __m128 {
        _mm_set_ps(0., 0., self.im, self.re)
    }
    
    #[inline(always)]
    pub(crate) unsafe fn unsafe_from (x: __m128) -> Self {
        let ptr = &x as *const __m128 as *const f32;
        Self::new(*ptr, *ptr.add(1))
    }
}