use std::fmt::Debug;
use crate::{vec::*, mat::*};
use super::{Complxf, Complxd};

macro_rules! impl_debug_complex {
    ($($target:ident),+) => {
        $(
            impl Debug for $target {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($target))
                        .field("re", &self.re())
                        .field("im", &self.im())
                        .finish()
                }
            }
        )*
    };
}

macro_rules! impl_debug2 {
    ($($target:ident),+) => {
        $(
            impl Debug for $target {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($target))
                        .field("x", &self.x())
                        .field("y", &self.y())
                        .finish()
                }
            }
        )*
    };
}

macro_rules! impl_debug3 {
    ($($target:ident),+) => {
        $(
            impl Debug for $target {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($target))
                        .field("x", &self.x())
                        .field("y", &self.y())
                        .field("z", &self.z())
                        .finish()
                }
            }
        )*
    };
}

macro_rules! impl_debug4 {
    ($($target:ident),+) => {
        $(
            impl Debug for $target {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($target))
                        .field("x", &self.x())
                        .field("y", &self.y())
                        .field("z", &self.z())
                        .field("w", &self.w())
                        .finish()
                }
            }
        )*
    };
}

impl_debug_complex!(
    Complxf, Complxd
);

impl_debug2!(
    EucVecf2, EucVecd2,
    Matf2, Matd2
);

impl_debug3!(
    EucVecf3, EucVecd3,
    Matf3, Matd3
);

impl_debug4!(
    EucVecf4, EucVecd4
);