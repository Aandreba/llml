use std::{ops::Div, intrinsics::transmute};
use crate::others::Hypot;

//use crate::others::Complx;
import_derives!();

pub type EucVecf2 = EucVec2<f32>;
pub type EucVecd2 = EucVec2<f64>;
pub type EucVeci2 = EucVec2<i32>;
pub type EucVecu2 = EucVec2<u32>;
pub type EucVecl2 = EucVec2<i64>;

/// Euclidian Vector of 2 values
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct EucVec2<T> {
    pub x: T,
    pub y: T
}

impl<T> EucVec2<T>  {
    pub fn new (x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_array (array: [T;2]) -> Self where T: Copy {
        unsafe { *(&array as *const [T;2] as *const Self) }
    }
}

/*
impl<T> Into<Complx<T>> for EucVec2<T> {
    fn into(self) -> Complx<T> {
        Complx::new(self.x, self.y)
    }
}

impl<T> From<Complx<T>> for EucVec2<T> {
    fn from(x: Complx<T>) -> EucVec2<T> {
        EucVec2::new(x.re, x.im)
    }
}*/