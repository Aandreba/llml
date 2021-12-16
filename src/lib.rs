#![feature(
    trusted_len, try_trait_v2, try_trait_v2_residual, 
    maybe_uninit_slice, maybe_uninit_uninit_array, maybe_uninit_array_assume_init,
    array_zip
)]

// UTILS
mod extra;
pub use extra::array;

// EXPORT
pub mod vec;
pub mod mat;

/*
    "x86"
    "x86_64"
    "mips"
    "powerpc"
    "powerpc64"
    "arm"
    "aarch64"
*/