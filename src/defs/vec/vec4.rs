import_derives!();

pub type EucVecf4 = EucVec4<f32>;
pub type EucVecd4 = EucVec4<f64>;
pub type EucVeci4 = EucVec4<i32>;
pub type EucVecu4 = EucVec4<u32>;
pub type EucVecl4 = EucVec4<i64>;

/// Euclidian Vector of 4 values
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct EucVec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> EucVec4<T>  {
    pub fn new (x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}