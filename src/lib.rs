#![feature(once_cell, concat_idents, core_intrinsics, set_ptr_value, portable_simd, trivial_bounds, stdsimd)]
#![cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), feature(stdarch))]

#![feature(extern_types, simd_ffi)]
//#![cfg_attr(target_feature = "avf512fp16", feature(extern_types))]

use std::arch::x86_64::_mm256_cvtneps_pbh;

use cfg_if::cfg_if;

macro_rules! flat_mod {
    ($($i:ident),*) => {
        $(
            mod $i;
            pub use self::$i::*;
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

import_derives!();
pub mod traits;
flat_mod!(generics);

cfg_if! {
    if #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))] {
        flat_mod!(arm);
    } else if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))] {
        flat_mod!(x86);
    } else {
        compile_error!("LLML not available for this target");
    }
}

/// Polar coordinates
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
pub struct Polar<T> {
    pub radius: T,
    pub angle: T
}

impl<T> Polar<T> {
    pub fn new (radius: T, angle: T) -> Self {
        Self { radius, angle }
    }
}