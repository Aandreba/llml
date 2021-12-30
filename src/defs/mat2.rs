use cfg_if::cfg_if;
use crate::EucVec2;

pub type Matf2 = EucVec2<f32>;
pub type Matd2 = EucVec2<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat2<T> {
                pub x: crate::EucVec2<T>,
                pub y: crate::EucVec2<T>
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat2<T> {
                pub x: crate::EucVec2<T>,
                pub y: crate::EucVec2<T>
            }
        }
    }
}

pub use def::*;

impl<T> Mat2<T>  {
    pub fn new (x: EucVec2<T>, y: EucVec2<T>) -> Self {
        Self { x, y }
    }
}