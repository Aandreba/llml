x86_use!();
use std::{ops::*, ptr::addr_of};
use crate::{vec::EucVec3f, others::Zero};

macro_rules! impl_matf3 {
    ($($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for Mat3f {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    let vec = unsafe { concat_idents!(_mm256_, $fun, _ps)(self.0, rhs.0) };
                    let scal = self.1.$fun(rhs.1);
                    Self(vec, scal)
                }
            }

            impl_matf3_scal!($trait, $fun);
        )*
    };
}

macro_rules! impl_matf3_scal {
    ($($trait:ident, $fun:ident),+) => {
        $(
            impl $trait<f32> for Mat3f {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: f32) -> Self::Output {
                    let vec;
                    unsafe {
                        let fill = _mm256_set1_ps(rhs);
                        vec = concat_idents!(_mm256_, $fun, _ps)(self.0, fill);
                    }

                    let scal = self.1.$fun(rhs);
                    Self(vec, scal)
                }
            }

            impl $trait<Mat3f> for f32 {
                type Output = Mat3f;

                #[inline(always)]
                fn $fun (self, rhs: Mat3f) -> Self::Output {
                    let vec;
                    unsafe {
                        let fill = _mm256_set1_ps(self);
                        vec = concat_idents!(_mm256_, $fun, _ps)(fill, rhs.0);
                    }

                    let scal = self.$fun(rhs.1);
                    Mat3f(vec, scal)
                }
            }
        )*
    };
}

#[derive(Copy)]
#[repr(C, align(64))]
pub struct Mat3f (pub(crate) __m256, pub(crate) f32);

impl_matf3!(
    Add, add,
    Sub, sub
);

impl_matf3_scal!(
    Mul, mul,
    Div, div
);

impl Mat3f {
    const VEC_MASK : __m128 = unsafe { *(&[0, u32::MAX, u32::MAX, u32::MAX] as *const [u32;4] as *const __m128) };

    #[inline]
    pub fn new (a: [f32;9]) -> Self {
        Self(
            unsafe { _mm256_set_ps(
                a[7], a[6], a[5], a[4],
                a[3], a[2], a[1], a[0]
            ) },
            a[8]
        )
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn x (&self) -> EucVec3f {
        unsafe { 
            EucVec3f(_mm_and_ps(_mm256_extractf128_ps(self.0, 0), Self::VEC_MASK))
        }
    }

    #[inline(always)]
    pub fn xx (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(self.0) }
    }

    #[inline(always)]
    pub fn xy (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(1, 1, 1, 1))) }
    }

    #[inline(always)]
    pub fn xz (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(2, 2, 2, 2))) }
    }

    #[inline(always)]
    pub fn y (&self) -> EucVec3f {
        unsafe {
            let ptr : *const f32 = addr_of!(self.0).cast();
            EucVec3f(_mm_and_ps(_mm_loadu_ps(ptr.add(3)), Self::VEC_MASK))
        }
    }

    #[inline(always)]
    pub fn yx (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(3, 3, 3, 3))) }
    }

    #[inline(always)]
    pub fn yy (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(4, 4, 4, 4))) }
    }

    #[inline(always)]
    pub fn yz (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(5, 5, 5, 5))) }
    }

    #[inline(always)]
    pub fn z (&self) -> EucVec3f {
        unsafe {
            EucVec3f( _mm_shuffle_ps(_mm256_extractf128_ps(self.0, 1), _mm_set_ps(0., 0., 0., self.1), _MM_SHUFFLE(1, 0, 1, 0)))
        }
    }

    #[inline(always)]
    pub fn zx (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(6, 6, 6, 6))) }
    }

    #[inline(always)]
    pub fn zy (&self) -> f32 {
        unsafe { _mm256_cvtss_f32(_mm256_shuffle_ps(self.0, self.0, _MM_SHUFFLE(7, 7, 7, 7))) }
    }

    #[inline(always)]
    pub fn zz (&self) -> f32 {
        self.1
    }

    #[inline(always)]
    pub fn scal_mul (self, rhs: Self) -> Self {
        let vec = unsafe { _mm256_mul_ps(self.0, rhs.0) };
        let scal = self.1 * rhs.1;
        Self(vec, scal)
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        let vec = unsafe { _mm256_div_ps(self.0, rhs.0) };
        let scal = self.1 / rhs.1;
        Self(vec, scal)
    }

    #[inline(always)]
    pub fn tr (self) -> f32 {
        EucVec3f::new([self.xx(), self.yy(), self.zz()]).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        todo!()
    }

    /// Performs the inverse of the matrix, returning ```None``` if it doesn't have one
    #[inline(always)]
    pub fn inv (self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None
        }

        Some(unsafe { self._inv(det) })
    }

    /// Performs the inverse without checking if the determinant is zero.\
    /// The use of this method is prefered if you're certain the matrix has an inverse,
    /// since it could be faster
    #[inline(always)]
    pub unsafe fn inv_unsafe (self) -> Self {
        self._inv(self.det())
    }

    #[inline(always)]
    unsafe fn _inv (self, det: f32) -> Self {
        todo!()
    }
}

impl Clone for Mat3f {
    fn clone(&self) -> Self {
        let vec = unsafe { _mm256_load_ps(addr_of!(self.0).cast()) };
        Self(vec, self.1)
    }
}