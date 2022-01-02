import_derives!();

use std::ops::{Add, Neg};
use crate::{vec::EucVec2, others::Zero};

pub type Matf2 = Mat2<f32>;
pub type Matd2 = Mat2<f64>;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct Mat2<T> {
    pub x: EucVec2<T>,
    pub y: EucVec2<T>
}

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

    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None;
        }

        Some(Self::of_values(self.y.y, -self.x.y, -self.y.x, self.x.x) / det)
    }
}

impl Matd2 {
    pub fn of_rot (rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::of_values(cos, -sin, sin, cos)
    }
}