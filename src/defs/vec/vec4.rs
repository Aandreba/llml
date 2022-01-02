import_derives!();

pub type EucVecf4 = EucVec4<f32>;
pub type EucVecd4 = EucVec4<f64>;

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

impl EucVecf4 {
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