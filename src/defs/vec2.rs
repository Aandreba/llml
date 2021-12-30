use cfg_if::cfg_if;

pub type EucVecf2 = EucVec2<f32>;
pub type EucVecd2 = EucVec2<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct EucVec2<T> {
                pub x: T,
                pub y: T
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct EucVec2<T> {
                pub x: T,
                pub y: T
            }
        }
    }
}

pub use def::*;

impl<T> EucVec2<T>  {
    pub fn new (x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl EucVecf2 {
    #[inline(always)]
    pub fn sum (self) -> f32 {
        self.x + self.y
    }

    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn unit (self) -> Self {
        self / self.norm()
    }
}