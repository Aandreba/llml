use std::{simd::{Simd}};
use crate::vec::EucVec3;
use crate::utils::copy_slice;

macro_rules! into {
    () => {
        into!(
            0,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize
        ); 
        
        into!(
            0.,
            f32, f64
        ); 
    };

    ($zero:literal, $($target:ident),+) => {
        $(
            impl EucVec3<$target> {
                #[inline(always)]
                pub(crate) unsafe fn into_simd (self) -> Simd<$target,4> {
                    Simd::from_array([self.x, self.y, self.z, $zero])
                } 
            
                #[inline(always)]
                pub(crate) unsafe fn from_simd (x: Simd<$target,4>) -> Self {
                    Self::from_array(copy_slice::<$target,4,3>(x.as_array()))
                }
            }

            impl Into<Simd<$target,4>> for EucVec3<$target> {
                fn into(self) -> Simd<$target,4> {
                    unsafe { self.into_simd() }
                }
            }
            
            impl From<Simd<$target,4>> for EucVec3<$target> {
                fn from(x: Simd<$target,4>) -> Self {
                    unsafe { Self::from_simd(x) }
                }
            }
        )*
    }
}

simd_map!(EucVec3);
into!();