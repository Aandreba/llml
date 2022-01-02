import_derives!();

pub type EucVecf2 = EucVec2<f32>;
pub type EucVecd2 = EucVec2<f64>;

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
}

impl EucVecf2 {
    /// Summation of all the values inside the vector
    #[inline(always)]
    pub fn sum (self) -> f32 {
        self.x + self.y
    }

    /// Returns ```norm(self)^2```
    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }

    /// Vector length / norm
    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Unit vector
    pub fn unit (self) -> Self {
        self / self.norm()
    }
}