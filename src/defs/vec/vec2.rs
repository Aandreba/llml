use crate::others::Complx;

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

impl<T> Into<Complx<T>> for EucVec2<T> {
    fn into(self) -> Complx<T> {
        Complx::new(self.x, self.y)
    }
}

impl<T> From<Complx<T>> for EucVec2<T> {
    fn from(x: Complx<T>) -> EucVec2<T> {
        EucVec2::new(x.re, x.im)
    }
}