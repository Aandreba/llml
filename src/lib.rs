#![feature(
    trusted_len, try_trait_v2, try_trait_v2_residual, 
    maybe_uninit_slice, maybe_uninit_uninit_array, maybe_uninit_array_assume_init,
    array_zip, generic_const_exprs, toowned_clone_into, concat_idents, step_trait,
    tuple_indexing
)]

// UTILS
mod extra;
pub use extra::array;

// EXPORT
pub mod poly;
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