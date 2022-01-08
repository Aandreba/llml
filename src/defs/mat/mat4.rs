import_derives!();
use crate::vec::{EucVec4};

pub type Matf4 = Mat4<f32>;
pub type Matd4 = Mat4<f64>;
pub type Mati4 = Mat4<i32>;
pub type Matu4 = Mat4<u32>;
pub type Matl4 = Mat4<i64>;

/// 4x4 Matrix
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct Mat4<T> {
    pub x: EucVec4<T>,
    pub y: EucVec4<T>,
    pub z: EucVec4<T>,
    pub w: EucVec4<T>
}

impl<T> Mat4<T>  {
    pub fn new (x: EucVec4<T>, y: EucVec4<T>, z: EucVec4<T>, w: EucVec4<T>) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_values (
        xx: T, xy: T, xz: T, xw: T,
        yx: T, yy: T, yz: T, yw: T,
        zx: T, zy: T, zz: T, zw: T,
        wx: T, wy: T, wz: T, ww: T
    ) -> Self {
        Self { 
            x: EucVec4::new(xx, xy, xz, xw),
            y: EucVec4::new(yx, yy, yz, yw),
            z: EucVec4::new(zx, zy, zz, zw),
            w: EucVec4::new(wx, wy, wz, ww)
        }
    }

    pub fn from_array (array: [T;16]) -> Self where T: Copy {
        let ptr = &array as *const [T;16] as *const EucVec4<T>;
        unsafe {
            Self { 
                x: *ptr,
                y: *ptr.add(1),
                z: *ptr.add(2),
                w: *ptr.add(3)
            }
        }
    } 

    /// Matrix transpose
    #[inline(always)]
    pub fn transp (self) -> Self {
        Self::from_values(
            self.x.x, self.y.x, self.z.x, self.w.x,
            self.x.y, self.y.y, self.z.y, self.w.y,
            self.x.z, self.y.z, self.z.z, self.w.z,
            self.x.w, self.y.w, self.z.w, self.w.w
        )
    }
}