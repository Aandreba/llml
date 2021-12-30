use cfg_if::cfg_if;

pub type EucVecf3 = EucVec3<f32>;
pub type EucVecd3 = EucVec3<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct EucVec3<T> {
                pub x: T,
                pub y: T,
                pub z: T
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct EucVec3<T> {
                pub x: T,
                pub y: T,
                pub z: T
            }
        }
    }
}

pub use def::*;

impl<T> EucVec3<T>  {
    pub fn new (x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/*
impl EucVecf3 {
    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }
}
*/