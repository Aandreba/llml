use cfg_if::cfg_if;

pub type EucVecf4 = EucVec4<f32>;
pub type EucVecd4 = EucVec4<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct EucVec4<T> {
                pub x: T,
                pub y: T,
                pub z: T,
                pub w: T
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(align(16))]
            pub struct EucVec4<T> {
                pub x: T,
                pub y: T,
                pub z: T,
                pub w: T
            }
        }
    }
}

pub use def::*;

impl<T> EucVec4<T>  {
    pub fn new (x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl EucVecf4 {
    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }
}
