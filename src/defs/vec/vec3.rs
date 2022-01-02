import_derives!();

pub type EucVecf3 = EucVec3<f32>;
pub type EucVecd3 = EucVec3<f64>;

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

impl EucVecf3 {
    /// Returns ```norm(self)^2```
    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }

    /// Vector length / norm
    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.norm2().sqrt()
    }

    /// Unit vector
    #[inline(always)]
    pub fn unit (self) -> Self {
        self / self.norm()
    }
}