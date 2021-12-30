macro_rules! use_arch_x86 {
    ($($i:ident),*) => {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::{$($i,)*};

        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::{$($i,)*};
    };
}

macro_rules! impl_vecf {
    ($name:ident, $cast:expr, $from:expr) => {
        use crate::$name;
        use_arch_x86!(__m128, _mm_add_ps, _mm_sub_ps, _mm_mul_ps, _mm_div_ps);

        map_to_trait!($name, Add, add, |x: Self, y: Self| Self::unsafe_from(_mm_add_ps(x.casted(), y.casted())));
        map_to_trait!($name, Sub, sub, |x: Self, y: Self| Self::unsafe_from(_mm_sub_ps(x.casted(), y.casted())));
        map_to_trait!($name, Mul, mul, |x: Self, y: Self| Self::unsafe_from(_mm_mul_ps(x.casted(), y.casted())));
        map_to_trait!($name, Div, div, |x: Self, y: Self| Self::unsafe_from(_mm_div_ps(x.casted(), y.casted())));

        map_to_trait!($name, Add, f32, add, $name, |x: Self, y: f32| Self::unsafe_from(_mm_add_ps(x.casted(), transmute((y,y,y,y)))));
        map_to_trait!(f32, Add, $name, add, $name, |x: Self, y: $name| $name::unsafe_from(_mm_add_ps(transmute((x,x,x,x)), y.casted())));

        map_to_trait!($name, Sub, f32, sub, $name, |x: Self, y: f32| Self::unsafe_from(_mm_sub_ps(x.casted(), transmute((y,y,y,y)))));
        map_to_trait!(f32, Sub, $name, sub, $name, |x: Self, y: $name| $name::unsafe_from(_mm_sub_ps(transmute((x,x,x,x)), y.casted())));

        map_to_trait!($name, Mul, f32, mul, $name, |x: Self, y: f32| Self::unsafe_from(_mm_mul_ps(x.casted(), transmute((y,y,y,y)))));
        map_to_trait!(f32, Mul, $name, mul, $name, |x: Self, y: $name| $name::unsafe_from(_mm_mul_ps(transmute((x,x,x,x)), y.casted())));

        map_to_trait!($name, Div, f32, div, $name, |x: Self, y: f32| Self::unsafe_from(_mm_div_ps(x.casted(), transmute((y,y,y,y)))));
        map_to_trait!(f32, Div, $name, div, $name, |x: Self, y: $name| $name::unsafe_from(_mm_div_ps(transmute((x,x,x,x)), y.casted())));
        
        impl $name {
            #[inline(always)]
            pub(crate) unsafe fn casted (self) -> __m128 {
                $cast(self)
            }
            
            #[inline(always)]
            pub(crate) unsafe fn unsafe_from (x: __m128) -> Self {
                $from(x)
            }
        }
    };
}

flat_mod!(vec2, vec3, vec4);
flat_mod!(mat2);