use crate::{mat::{Matf2, Matd2, Matf3, Matd3}, vec::*};
use std::ops::*;

macro_rules! impl_default2 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl Default for $target {
                fn default () -> Self {
                    $target::new(<[$ty;4]>::default())
                }
            }
        )*
    };
}

macro_rules! impl_default3 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl Default for $target {
                fn default () -> Self {
                    $target::new(<[$ty;9]>::default())
                }
            }
        )*
    };
}

macro_rules! impl_rot2 {
    ($ty:ident) => {
        #[inline]
        /// Returns a matrix thet represents the specified rotation (in radians)
        pub fn from_rot (angle: $ty) -> Self {
            let (sin, cos) = angle.sin_cos();
            Self::new([cos, -sin, sin, cos])
        }
    };
}

macro_rules! impl_rot3 {
    ($ty:ident) => {
        #[inline]
        /// Returns a matrix thet represents the specified rotation (in radians)
        pub fn from_rot (roll: $ty, pitch: $ty, yaw: $ty) -> Self {
            let (s1, c1) = roll.sin_cos();
            let (s2, c2) = pitch.sin_cos();
            let (s3, c3) = yaw.sin_cos();
            
            let c3s2 = c3 * s2;
            let c1s3 = c1 * s3;
            let s1s3 = s1 * s3;

            Self::new([
                c2 * c3, c3s2 * s1 + c1s3, s1s3 - c1 * c3s2,
                -c2 * s3, c1 * c3 - s1s3 * s2, c3 * s1 + c1s3 * s2,
                s2, -c2 * s1, c1 * c2
            ])
        }
    };
}

macro_rules! impl_assign_mat {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl_assign!(
                $target, 
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul
            );
    
            impl_assign!(
                1, $target, $ty,
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul,
                DivAssign, div_assign, div
            );
        )*
    }
}

impl_default2!(
    Matf2, f32,
    Matd2, f64
);

impl_default3!(
    Matf3, f32,
    Matd3, f64
);

impl_assign_mat!(
    Matf2, f32,
    Matf3, f32,
    Matd2, f64,
    Matd3, f64
);

impl Matf2 { impl_rot2!(f32); }
impl Matd2 { impl_rot2!(f64); }
impl Matf3 { impl_rot3!(f32); }
impl Matd3 { impl_rot3!(f64); }