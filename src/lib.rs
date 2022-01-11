#![feature(once_cell, concat_idents, core_intrinsics, set_ptr_value, portable_simd, trivial_bounds, generic_const_exprs)]
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
        use randerive::{Rand};
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

pub mod traits;

/*cfg_if! {
    if #[cfg(feature = "llml_naive")] {
        flat_mod!(naive);
    } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        compile_error!("Hi");
        flat_mod!(aarch64);
    } else if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))] {
        flat_mod!(x86);
    } else {
        flat_mod!(naive);
    }
}*/

flat_mod!(arm);

#[cfg(feature = "llml_derive")]
pub mod derive {
    pub use llml_derive_crate::*;
}