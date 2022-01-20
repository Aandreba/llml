#![feature(once_cell, concat_idents, core_intrinsics, set_ptr_value, portable_simd, trivial_bounds, stdsimd)]
#![cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), feature(stdarch))]

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

macro_rules! mmod {
    ($($i:ident),+) => {
        $(
            pub(crate) mod $i;
        )*
    };
}

macro_rules! import_derives {
    () => {
        #[cfg(feature = "llml_serde")]
        use serde::{Serialize, Deserialize};

        #[cfg(feature = "llml_rand")]
        use randerive::Rand;
    };
}

pub(crate) mod generics;
pub(crate) mod polar;
pub(crate) mod traits;

#[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
pub(crate) mod arm;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
pub(crate) mod x86;

mod defs;
pub use defs::*;