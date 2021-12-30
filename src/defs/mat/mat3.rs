use cfg_if::cfg_if;
use crate::vec::EucVec3;

pub type Matf3 = Mat3<f32>;
pub type Matd3 = Mat3<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat3<T> {
                pub x: crate::vec::EucVec3<T>,
                pub y: crate::vec::EucVec3<T>,
                pub z: crate::vec::EucVec3<T>
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Mat3<T> {
                pub x: crate::vec::EucVec3<T>,
                pub y: crate::vec::EucVec3<T>,
                pub z: crate::vec::EucVec3<T>
            }
        }
    }
}

pub use def::*;

impl<T> Mat3<T>  {
    pub fn new (x: EucVec3<T>, y: EucVec3<T>, z: EucVec3<T>) -> Self {
        Self { x, y, z }
    }

    pub fn of_values (
        xx: T, xy: T,  xz: T,
        yx: T, yy: T, yz: T,
        zx: T, zy: T, zz: T
    ) -> Self {
        Self { 
            x: EucVec3::new(xx, xy, xz),
            y: EucVec3::new(yx, yy, yz),
            z: EucVec3::new(zx, zy, zz)
        }
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        Self::of_values(
            self.x.x, self.y.x, self.z.x, 
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z
        )
    }
}

impl Matf3 {
    pub fn of_rot (roll: f32, pitch: f32, yaw: f32) -> Self {
        let (sy, cy) = roll.sin_cos();
        let (sb, cb) = pitch.sin_cos();
        let (sa, ca) = yaw.sin_cos();

        let sbsy = sb * sy;
        let sbcy = sb * cy;

        Self::of_values(
            ca * cb, ca.mul_add(sbsy, -sa * cy), ca.mul_add(sbcy, sa * sy), 
            sa * cb, sa.mul_add(sbsy, ca * cy), sa.mul_add(sbcy, -ca * sy),
            -sb, cb * sy, cb * cy
        )
    }
}

impl Matd3 {
    pub fn of_rot (roll: f64, pitch: f64, yaw: f64) -> Self {
        let (sy, cy) = roll.sin_cos();
        let (sb, cb) = pitch.sin_cos();
        let (sa, ca) = yaw.sin_cos();

        let sbsy = sb * sy;
        let sbcy = sb * cy;

        Self::of_values(
            ca * cb, ca.mul_add(sbsy, -sa * cy), ca.mul_add(sbcy, sa * sy), 
            sa * cb, sa.mul_add(sbsy, ca * cy), sa.mul_add(sbcy, -ca * sy),
            -sb, cb * sy, cb * cy
        )
    }
}