#![feature(once_cell, concat_idents)]
use cfg_if::cfg_if;

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

flat_mod!(defs);
cfg_if! {
    if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        flat_mod!(aarch64);
    }
}