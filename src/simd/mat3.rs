use std::{simd::{Simd}};
use crate::{mat::{Mat3}, utils::{copy_slice, copy_slice_w_offset, copy_with_padding}, vec::{EucVec3}};

macro_rules! into_simd {
    () => {
        into_simd!(
            0,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize
        );

        into_simd!(
            0., 
            f32, f64
        );
    };

    ($zero:literal, $($ty:ident),+) => {
        $(
            impl Mat3<$ty> {
                #[inline(always)]
                pub(crate) unsafe fn into_simd (self) -> Simd<$ty,16> {
                    Simd::from_array([
                        self.x.x, self.x.y, self.x.z,
                        self.y.x, self.y.y, self.y.z,
                        self.z.x, self.z.y, self.z.z,
                        $zero, $zero, $zero, $zero, $zero, $zero, $zero
                    ])
                }

                #[inline(always)]
                pub(crate) unsafe fn from_simd (x: Simd<$ty,16>) -> Self {
                    Mat3::from_array(copy_slice::<$ty,16,9>(x.as_array()))
                }
            } 

            impl Into<Simd<$ty,16>> for Mat3<$ty> {
                fn into(self) -> Simd<$ty,16> {
                    unsafe { self.into_simd() }
                }
            }
            
            impl From<Simd<$ty,16>> for Mat3<$ty> {
                fn from(x: Simd<$ty,16>) -> Self {
                    unsafe { Self::from_simd(x) }
                }
            }
        )*
    };
}

macro_rules! impl_mul {
    () => {
        impl_mul!(
            0,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize
        );

        impl_mul!(
            0.,
            f32, f64
        );
    };

    ($zero:literal, $($ty:ident),+) => {
        $(
            impl Mul for Mat3<$ty> {
                type Output = Self;
            
                fn mul(self, rhs: Self) -> Self::Output {
                    let a = Simd::from_array([
                        self.x.x, self.x.x, self.x.x,
                        self.y.x, self.y.x, self.y.x,
                        self.z.x, self.z.x, self.z.x,
            
                        self.x.y, self.x.y, self.x.y,
                        self.y.y, self.y.y, self.y.y,
                        self.z.y, self.z.y, self.z.y,
            
                        self.x.z, self.x.z, self.x.z,
                        self.y.z, self.y.z, self.y.z,
                        self.z.z, self.z.z, self.z.z,
            
                        $zero, $zero, $zero, $zero, $zero
                    ]);
            
                    let b = Simd::from_array([
                        rhs.x.x, rhs.x.y, rhs.x.z,
                        rhs.x.x, rhs.x.y, rhs.x.z,
                        rhs.x.x, rhs.x.y, rhs.x.z,
            
                        rhs.y.x, rhs.y.y, rhs.y.z,
                        rhs.y.x, rhs.y.y, rhs.y.z,
                        rhs.y.x, rhs.y.y, rhs.y.z,
            
                        rhs.z.x, rhs.z.y, rhs.z.z,
                        rhs.z.x, rhs.z.y, rhs.z.z,
                        rhs.z.x, rhs.z.y, rhs.z.z,
            
                        $zero, $zero, $zero, $zero, $zero
                    ]);
            
                    let m1 = a * b;
            
                    unsafe {
                        let s1 = copy_slice::<$ty,32,9>(m1.as_array());
                        let s2 = copy_slice_w_offset::<$ty,32,9>(m1.as_array(), 9);
                        let s3 = copy_slice_w_offset::<$ty,32,9>(m1.as_array(), 18);
            
                        let s1 = Simd::from_array(copy_with_padding::<$ty,9,16>(s1));
                        let s2 = Simd::from_array(copy_with_padding::<$ty,9,16>(s2));
                        let s3 = Simd::from_array(copy_with_padding::<$ty,9,16>(s3));
            
                        Self::from_simd(s1 + s2 + s3)
                    }
                }
            }
            
            impl Mul<EucVec3<$ty>> for Mat3<$ty> {
                type Output = EucVec3<$ty>;
            
                fn mul(self, rhs: EucVec3<$ty>) -> Self::Output {
                    let a = Simd::from_array([
                        self.x.x, self.y.x, self.z.x,
                        self.x.y, self.y.y, self.z.y,
                        self.x.z, self.y.z, self.z.z,
                        $zero, $zero, $zero, $zero, $zero, $zero, $zero
                    ]);
            
                    let b = Simd::from_array([
                        rhs.x, rhs.x, rhs.x,
                        rhs.y, rhs.y, rhs.y,
                        rhs.z, rhs.z, rhs.z,
                        $zero, $zero, $zero, $zero, $zero, $zero, $zero
                    ]);
            
                    let m1 = a * b;
                    unsafe {
                        let s1 = copy_slice::<$ty,16,3>(m1.as_array());
                        let s2 = copy_slice_w_offset::<$ty,16,3>(m1.as_array(), 3);
                        let s3 = copy_slice_w_offset::<$ty,16,3>(m1.as_array(), 6);
            
                        let s1 = Simd::from_array(copy_with_padding::<$ty,3,4>(s1));
                        let s2 = Simd::from_array(copy_with_padding::<$ty,3,4>(s2));
                        let s3 = Simd::from_array(copy_with_padding::<$ty,3,4>(s3));
            
                        EucVec3::<$ty>::from_simd(s1 + s2 + s3)
                    }
                }
            }
        )*
    };
}

simd_mat_map!(Mat3);
into_simd!();
impl_mul!();