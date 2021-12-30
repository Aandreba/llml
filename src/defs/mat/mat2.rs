use std::ops::Add;

use cfg_if::cfg_if;
use crate::vec::EucVec2;

pub type Matf2 = Mat2<f32>;
pub type Matd2 = Mat2<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat2<T> {
                pub x: crate::vec::EucVec2<T>,
                pub y: crate::vec::EucVec2<T>
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat2<T> {
                pub x: crate::vec::EucVec2<T>,
                pub y: crate::vec::EucVec2<T>
            }
        }
    }
}

pub use def::*;

impl<T> Mat2<T>  {
    pub fn new (x: EucVec2<T>, y: EucVec2<T>) -> Self {
        Self { x, y }
    }

    pub fn of_values (xx: T, xy: T, yx: T, yy: T) -> Self {
        Self { 
            x: EucVec2::new(xx, xy),
            y: EucVec2::new(yx, yy) 
        }
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        Self::of_values(self.x.x, self.y.x, self.x.y, self.y.y)
    }

    #[inline(always)]
    pub fn tr (self) -> <T as Add>::Output where T: Add {
        self.x.x + self.y.y
    }
}

impl Matf2 {
    pub fn of_rot (rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::of_values(cos, -sin, sin, cos)
    }
}

impl Matd2 {
    pub fn of_rot (rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::of_values(cos, -sin, sin, cos)
    }
}