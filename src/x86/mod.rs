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

        impl Eq for $target {}
        impl Copy for $target {}

        impl Div for $target {
            type Output = Self;

            #[inline(always)]
            fn div (self, rhs: Self) -> Self::Output {
                unsafe {
                    let div = concat_idents!(_m, $tag, _div_p, $sub)(self.0, rhs.0);
                    Self(concat_idents!(_m, $tag, _and_p, $sub)(Self::DIV_MASK, div))
                }
            }
        }

        impl Div<$ty> for $target {
            type Output = Self;

            #[inline(always)]
            fn div (self, rhs: $ty) -> Self::Output {
                self.div(Self::from_scal(rhs))
            }
        }

        impl Div<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn div (self, rhs: $target) -> Self::Output {
                $target::from_scal(self).div(rhs)
            }
        }

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
                self.$fun(Self::from_scal(rhs))
            }
        }

        impl $trait<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn $fun (self, rhs: $target) -> Self::Output {
                $target::from_scal(self).$fun(rhs)
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

        impl $target {
            #[inline(always)]
            #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
            pub fn norm2 (self) -> $ty {
                self.dot(self)
            }

            #[inline(always)]
            pub fn unit (self) -> Self {
                self / self.norm()
            }
        }

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

macro_rules! trait_map {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0.$fun(rhs.0))
                }
            }

            trait_map_scal!($target, $ty, $trait, $fun);
        )*
    };
}

macro_rules! trait_map_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0.$fun(rhs))
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self.$fun(rhs.0))
                }
            }
        )*
    };
}

x86_use!();
flat_mod!(complex, vec2, vec3, vec4, mat2);

#[cfg(target_feature = "sse2")]
flat_mod!(double);

cfg_if! {
    if #[cfg(all(feature = "llml_avx", target_feature = "avx"))] {
        flat_mod!(avx);
    } else {
        flat_mod!(sse);
    }
}

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

#[inline(always)]
pub(crate) unsafe fn _mm_combine_ps (a: __m128, b: __m128) -> __m128 {
    _mm_shuffle_ps(a, b, _MM_SHUFFLE(1, 0, 1, 0))
}

#[inline(always)]
pub(crate) unsafe fn _mm_low_ps (a: __m128) -> __m128 {
    _mm_movehl_ps(_mm_setzero_ps(), a)
}

#[inline(always)]
pub(crate) unsafe fn _mm_high_ps (a: __m128) -> __m128 {
    _mm_shuffle_ps(a, _mm_setzero_ps(), _MM_SHUFFLE(3, 2, 1, 0))
}

#[inline(always)]
pub(crate) unsafe fn _mm_low_high_ps (a: __m128) -> (__m128, __m128) {
    let zero = _mm_setzero_ps();
    (_mm_movehl_ps(zero, a), _mm_shuffle_ps(a, zero, _MM_SHUFFLE(3, 2, 1, 0)))
}