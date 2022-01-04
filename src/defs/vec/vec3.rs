use std::{simd::SimdElement, ops::Div};

use crate::others::{Hypot, Sqrt};

import_derives!();

pub type EucVecf3 = EucVec3<f32>;
pub type EucVecd3 = EucVec3<f64>;
pub type EucVeci3 = EucVec3<i32>;
pub type EucVecu3 = EucVec3<u32>;
pub type EucVecl3 = EucVec3<i64>;

/// Euclidian Vector of 3 values
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct EucVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> EucVec3<T>  {
    pub fn new (x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}