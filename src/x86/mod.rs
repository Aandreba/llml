use cfg_if::cfg_if;

macro_rules! x86_use {
    () => {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;

        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;
    };
}

macro_rules! impl_arith {
    ($target:ident, f32) => {
        type __m128s = __m128;
        impl_arith!($target, f32, s, m);
    };

    ($target:ident, f32, $tag:ident) => {
        impl_arith!($target, f32, s, $tag);
    };

    ($target:ident, f64) => {
        impl_arith!($target, f64, d, m);
    };

    ($target:ident, f64, $tag:ident) => {
        impl_arith!($target, f64, d, $tag);
    };

    ($target:ident, $ty:ident, $sub:ident, $tag:ident) => {        
        impl_arith!($target, $ty, $sub, Add, add, $tag);
        impl_arith!($target, $ty, $sub, Sub, sub, $tag);
        impl_arith!($target, $ty, $sub, Mul, mul, $tag);
        impl_arith!($target, $ty, $sub, Div, div, $tag);

        impl Eq for $target {}
        impl Copy for $target {}

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                0. - self
            }
        }

        impl Clone for $target {
            #[inline(always)]
            fn clone (&self) -> Self {
                unsafe {
                    Self(concat_idents!(_m, $tag, _load_p, $sub)(self as *const Self as *const $ty))
                }
            }
        }
    };

    ($target:ident, $ty:ident, $sub:ident, $trait:ident, $fun:ident, $tag:ident) => {
        impl $trait for $target {
            type Output = Self;

            #[inline(always)]
            fn $fun (self, rhs: Self) -> Self::Output {
                unsafe {
                    Self(concat_idents!(_m, $tag, _, $fun, _p, $sub)(self.0, rhs.0))
                }
            }
        }

        impl $trait<$ty> for $target {
            type Output = Self;

            #[inline(always)]
            fn $fun (self, rhs: $ty) -> Self::Output {
                self.$fun(Self::from_scalar(rhs))
            }
        }

        impl $trait<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn $fun (self, rhs: $target) -> Self::Output {
                $target::from_scalar(self).$fun(rhs)
            }
        }
    }
}

macro_rules! impl_arith_sse {
    ($target:ident, f32) => {
        impl_arith_sse!($target, f32, s);
    };

    ($target:ident, f64) => {
        impl_arith_sse!($target, f64, d);
    };

    ($target:ident, $ty:ident, $sub:ident) => {
        impl_arith!($target, $ty);

        impl PartialEq for $target {
            #[inline(always)]
            fn eq (&self, rhs: &Self) -> bool {
                unsafe {
                    *(&concat_idents!(_mm_cmpeq_p, $sub)(self.0, rhs.0) as *const concat_idents!(__m128, $sub) as *const u128) == u128::MAX
                }
            }
        }
    }
}

x86_use!();
flat_mod!(vec2, vec3, vec4);
flat_mod!(mat);

#[cfg(target_feature = "sse2")]
flat_mod!(double);

#[inline(always)]
pub(crate) unsafe fn _mm_sum_ps (lhs: __m128) -> f32 {
    #[cfg(target_feature = "sse3")]
    let shuf = _mm_movehdup_ps(lhs);
    #[cfg(not(target_feature = "sse3"))]
    let shuf = _mm_shuffle_ps(lhs, lhs, _MM_SHUFFLE(2, 3, 0, 1));

    let sums = _mm_add_ps(lhs, shuf);
    let shuf = _mm_movehl_ps(shuf, sums);
    let sums = _mm_add_ps(sums, shuf);
    
    _mm_cvtss_f32(sums)
}