use std::{simd::{SimdElement, Simd}};
use crate::{mat::{Mat2}, utils::{copy_slice, copy_slice_w_offset}, vec::EucVec2};

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
            impl Mul for Mat2<$ty> {
                type Output = Self;
            
                #[inline(always)]
                fn mul (self, rhs: Self) -> Self::Output {
                    let v1 = Simd::from_array([
                        self.x.x, self.x.x, self.y.x, self.y.x,
                        self.x.y, self.x.y, self.y.y, self.y.y
                    ]);
            
                    let v2 = Simd::from_array([
                        rhs.x.x, rhs.x.y, rhs.x.x, rhs.x.y,
                        rhs.y.x, rhs.y.y, rhs.y.x, rhs.y.y
                    ]);
            
                    let m1 = v1 * v2;
                    unsafe {
                        let v1 = Simd::from_array(copy_slice::<$ty,8,4>(m1.as_array()));
                        let v2 = Simd::from_array(copy_slice_w_offset::<$ty,8,4>(m1.as_array(), 4));
                        Self::from_simd(v1 + v2)
                    }
                }
            }

            impl Mul<EucVec2<$ty>> for Mat2<$ty> {
                type Output = EucVec2<$ty>;
            
                #[inline(always)]
                fn mul (self, rhs: EucVec2<$ty>) -> Self::Output {
                    let v1 = Simd::from_array([
                        self.x.x, self.y.x, self.x.y, self.y.y
                    ]);
            
                    let v2 = Simd::from_array([
                        rhs.x, rhs.x, rhs.y, rhs.y
                    ]);
            
                    let m1 = v1 * v2;
                    unsafe {
                        let v1 = Simd::from_array(copy_slice::<$ty,4,2>(m1.as_array()));
                        let v2 = Simd::from_array(copy_slice_w_offset::<$ty,4,2>(m1.as_array(), 2));
                        EucVec2::from_simd(v1 + v2)
                    }
                }
            }
        )*
    };
}

simd_mat_map!(Mat2);
impl_mul!();

impl<T: SimdElement> Mat2<T> {
    #[inline(always)]
    pub(crate) unsafe fn into_simd (self) -> Simd<T,4> {
        *(&self as *const Self as *const Simd<T,4>)
    } 

    #[inline(always)]
    pub(crate) unsafe fn from_simd (x: Simd<T,4>) -> Self {
        *(&x as *const Simd<T,4> as *const Self)
    }
}

impl<T: SimdElement> Into<Simd<T,4>> for Mat2<T> {
    fn into(self) -> Simd<T,4> {
        unsafe { self.into_simd() }
    }
}

impl <T: SimdElement> From<Simd<T,4>> for Mat2<T> {
    fn from(x: Simd<T,4>) -> Self {
        unsafe { Self::from_simd(x) }
    }
}