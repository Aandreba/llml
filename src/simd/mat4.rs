use std::{simd::{SimdElement, Simd}};
use crate::{mat::{Mat4}, utils::{copy_slice, copy_slice_w_offset}, vec::EucVec4};

macro_rules! impl_mul {
    () => {
        impl_mul!(
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize,
            f32, f64
        );
    };

    ($($ty:ident),+) => {
        $(
            impl Mul for Mat4<$ty> {
                type Output = Self;
            
                #[inline(always)]
                fn mul (self, rhs: Self) -> Self::Output {
                    let a = Simd::from_array([
                        self.x.x, self.x.x, self.x.x, self.x.x,
                        self.y.x, self.y.x, self.y.x, self.y.x,
                        self.z.x, self.z.x, self.z.x, self.z.x,
                        self.w.x, self.w.x, self.w.x, self.w.x,
             
                        self.x.y, self.x.y, self.x.y, self.x.y,
                        self.y.y, self.y.y, self.y.y, self.y.y,
                        self.z.y, self.z.y, self.z.y, self.z.y,
                        self.w.y, self.w.y, self.w.y, self.w.y,
            
                        self.x.z, self.x.z, self.x.z, self.x.z,
                        self.y.z, self.y.z, self.y.z, self.y.z,
                        self.z.z, self.z.z, self.z.z, self.z.z,
                        self.w.z, self.w.z, self.w.z, self.w.z,

                        self.x.w, self.x.w, self.x.w, self.x.w,
                        self.y.w, self.y.w, self.y.w, self.y.w,
                        self.z.w, self.z.w, self.z.w, self.z.w,
                        self.w.w, self.w.w, self.w.w, self.w.w
                    ]);
            
                    let b = Simd::from_array([
                        rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w,
                        rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w,
                        rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w,
                        rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w,
            
                        rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w,
                        rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w,
                        rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w,
                        rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w,
            
                        rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w,
                        rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w,
                        rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w,
                        rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w,

                        rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w,
                        rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w,
                        rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w,
                        rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w
                    ]);
            
                    let m1 = a * b;
                    unsafe {
                        let v1 = Simd::from_array(*copy_slice::<$ty,64,16>(m1.as_array()));
                        let v2 = Simd::from_array(*copy_slice_w_offset::<$ty,64,16>(m1.as_array(), 16));
                        let v3 = Simd::from_array(*copy_slice_w_offset::<$ty,64,16>(m1.as_array(), 32));
                        let v4 = Simd::from_array(*copy_slice_w_offset::<$ty,64,16>(m1.as_array(), 48));

                        Self::from_simd(v1 + v2 + v3 + v4)
                    }
                }
            }

            impl Mul<EucVec4<$ty>> for Mat4<$ty> {
                type Output = EucVec4<$ty>;
            
                #[inline(always)]
                fn mul (self, rhs: EucVec4<$ty>) -> Self::Output {
                    let a = Simd::from_array([
                        self.x.x, self.y.x, self.z.x, self.w.x,
                        self.x.y, self.y.y, self.z.y, self.w.y,
                        self.x.z, self.y.z, self.z.z, self.w.z,
                        self.x.w, self.y.w, self.z.w, self.w.w,
                    ]);
            
                    let b = Simd::from_array([
                        rhs.x, rhs.x, rhs.x, rhs.x,
                        rhs.y, rhs.y, rhs.y, rhs.y,
                        rhs.z, rhs.z, rhs.z, rhs.z,
                        rhs.w, rhs.w, rhs.w, rhs.w
                    ]);

                    let m1 = a * b;
                    unsafe {
                        let v1 = Simd::from_array(*copy_slice::<$ty,16,4>(m1.as_array()));
                        let v2 = Simd::from_array(*copy_slice_w_offset::<$ty,16,4>(m1.as_array(), 4));
                        let v3 = Simd::from_array(*copy_slice_w_offset::<$ty,16,4>(m1.as_array(), 8));
                        let v4 = Simd::from_array(*copy_slice_w_offset::<$ty,16,4>(m1.as_array(), 16));

                        EucVec4::<$ty>::from_simd(v1 + v2 + v3 + v4)
                    }
                }
            }
        )*
    };
}

simd_mat_map!(Mat4);
impl_mul!();

impl<T: SimdElement> Mat4<T> {
    #[inline(always)]
    pub(crate) unsafe fn into_simd (self) -> Simd<T,16> {
        *(&self as *const Self as *const Simd<T,16>)
    } 

    #[inline(always)]
    pub(crate) unsafe fn from_simd (x: Simd<T,16>) -> Self {
        *(&x as *const Simd<T,16> as *const Self)
    }
}

impl<T: SimdElement> Into<Simd<T,16>> for Mat4<T> {
    fn into(self) -> Simd<T,16> {
        unsafe { self.into_simd() }
    }
}

impl <T: SimdElement> From<Simd<T,16>> for Mat4<T> {
    fn from(x: Simd<T,16>) -> Self {
        unsafe { Self::from_simd(x) }
    }
}