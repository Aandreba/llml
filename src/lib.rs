#![feature(once_cell, concat_idents, core_intrinsics, stdarch)]
use cfg_if::cfg_if;
use static_assertions::{assert_eq_size, assert_eq_align};

macro_rules! flat_mod {
    ($($i:ident),*) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

macro_rules! map_to_trait {
    ($lhs:ident, $trait:ident, $fn:ident, $f:expr) => {
        map_to_trait!($lhs, $trait, $fn, Self, $f);
    };

    ($lhs:ident, $trait:ident, $fn:ident, $o:ident, $f:expr) => {
        map_to_trait!($lhs, $trait, Self, $fn, $o, $f);
    };

    ($lhs:ident, $trait:ident, $rhs:ident, $fn:ident, $o:ident, $f:expr) => {
        impl $trait<$rhs> for $lhs {
            type Output = $o;

            #[inline(always)]
            fn $fn (self, rhs: $rhs) -> $o {
                unsafe { $f(self, rhs) }
            }
        }
    };
}
macro_rules! assert_eq_layout {
    ($($x:ty, $y:ty),+) => {
        $(
            assert_eq_size!($x, $y);
            assert_eq_align!($x, $y);
        )*
    };
}

flat_mod!(defs);
cfg_if! {
    if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        flat_mod!(aarch64);
    } else if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))] {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::__m128;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::__m128;

        assert_eq_layout!(
            EucVecf4, __m128
        );

        flat_mod!(x86);
    } else {
        flat_mod!(naive);
    }
}