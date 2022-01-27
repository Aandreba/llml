x86_use!();
use crate::{traits::Zero, x86::{vec3::EucVec3f, vec2::EucVec2f, vec4::EucVec4f, _mm_low_high_ps, _mm_sum_ps, _mm_combine_ps}, mat::Mat3d};
use std::{ops::{Add, Sub, Mul, Div, Neg}, intrinsics::transmute};

macro_rules! impl_matf3 {
    () => {
        impl_matf3!(
            Add, add,
            Sub, sub
        );

        impl_matf3_scal!(
            Mul, mul,
            Div, div
        );
    };

    ($($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for Mat3f {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0.$fun(rhs.0), self.1.$fun(rhs.1), self.2.$fun(rhs.2))
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
                    unsafe {
                        let rhs = _mm_set1_ps(rhs);
                        let x = concat_idents!(_mm_, $fun, _ps)(self.0.0, rhs);
                        let y = concat_idents!(_mm_, $fun, _ps)(self.1.0, rhs);
                        let z = concat_idents!(_mm_, $fun, _ps)(self.2.0, rhs);
                        Self(EucVec3f(x), EucVec3f(y), EucVec3f(z))
                    }
                }
            }

            impl $trait<Mat3f> for f32 {
                type Output = Mat3f;

                #[inline(always)]
                fn $fun (self, rhs: Mat3f) -> Self::Output {
                    unsafe {
                        let sf = _mm_set1_ps(self);
                        let x = concat_idents!(_mm_, $fun, _ps)(sf, rhs.0.0);
                        let y = concat_idents!(_mm_, $fun, _ps)(sf, rhs.1.0);
                        let z = concat_idents!(_mm_, $fun, _ps)(sf, rhs.2.0);
                        Mat3f(EucVec3f(x), EucVec3f(y), EucVec3f(z))
                    }
                }
            }
        )*
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct Mat3f (pub(crate) EucVec3f, pub(crate) EucVec3f, pub(crate) EucVec3f);
impl_matf3!();

impl Neg for Mat3f {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mat3f {
    #[inline]
    pub fn new (a: [f32;9]) -> Self {
        Mat3f(
            EucVec3f::new([a[0], a[1], a[2]]),
            EucVec3f::new([a[3], a[4], a[5]]),
            EucVec3f::new([a[6], a[7], a[8]])
        )
    }

    #[inline(always)]
    pub fn transp (self) -> Self {
        Self::new([
            self.xx(), self.yx(), self.zx(),
            self.xy(), self.yy(), self.zy(),
            self.xz(), self.yz(), self.zz()
        ])
    }

    #[inline(always)]
    pub fn x (&self) -> EucVec3f {
        self.0
    }

    #[inline(always)]
    pub fn xx (&self) -> f32 {
        self.0.x()
    }

    #[inline(always)]
    pub fn xy (&self) -> f32 {
        self.0.y()
    }

    #[inline(always)]
    pub fn xz (&self) -> f32 {
        self.0.z()
    }

    #[inline(always)]
    pub fn y (&self) -> EucVec3f {
        self.1
    }

    #[inline(always)]
    pub fn yx (&self) -> f32 {
        self.1.x()
    }

    #[inline(always)]
    pub fn yy (&self) -> f32 {
        self.1.y()
    }

    #[inline(always)]
    pub fn yz (&self) -> f32 {
        self.1.z()
    }

    #[inline(always)]
    pub fn z (&self) -> EucVec3f {
        self.2
    }

    #[inline(always)]
    pub fn zx (&self) -> f32 {
        self.2.x()
    }

    #[inline(always)]
    pub fn zy (&self) -> f32 {
        self.2.y()
    }

    #[inline(always)]
    pub fn zz (&self) -> f32 {
        self.2.z()
    }

    #[inline(always)]
    pub fn scal_mul (self, rhs: Self) -> Self {
        Self (
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2
        )
    }

    #[inline(always)]
    pub fn scal_div (self, rhs: Self) -> Self {
        Self (
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2
        )
    }

    #[inline(always)]
    pub fn tr (self) -> f32 {
        EucVec3f::new([self.0.x(), self.1.y(), self.2.z()]).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        // Negation
        let neg = -self.0;

        // Subdets 1 & 2
        let v0 = EucVec4f::new([self.0.x(), neg.y(), neg.x(), self.0.y()]);
        let v1 = EucVec4f::new([self.1.y(), self.1.x(), self.1.z(), self.1.z()]);
        let v2 = EucVec4f::new([self.2.z(), self.2.z(), self.2.y(), self.2.x()]);
        let m1 = v0 * v1 * v2;

        let s1;
        unsafe {
            let (v1, v2) = _mm_low_high_ps(m1.0);
            s1 = _mm_add_ps(v1, v2);
        }

        // Subdet 3
        let v5 = EucVec2f::new([self.0.z(), neg.z()]);
        let v6 = EucVec2f::new([self.1.x(), self.1.y()]);
        let v7 = EucVec2f::new([self.2.y(), self.2.x()]);
        let m2 = v5 * v6 * v7;

        unsafe {
            _mm_sum_ps(_mm_combine_ps(s1, m2.0))
        }
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
        // TODO
        let det = _mm_set1_ps(det);

        // Row #1
        let v1 = EucVec3f::new([self.1.y(), self.0.z(), self.0.y()]);
        let v2 = EucVec3f::new([self.2.z(), self.2.y(), self.1.z()]);
        let m1 = v1 * v2;

        let v1 = EucVec3f::new([self.1.z(), self.0.y(), self.0.z()]);
        let v2 = EucVec3f::new([self.2.y(), self.2.z(), self.1.y()]);
        let m2 = v1 * v2;

        let s1 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        // Row #2
        let v1 = EucVec3f::new([self.1.z(), self.0.x(), self.0.z()]);
        let v2 = EucVec3f::new([self.2.x(), self.2.z(), self.1.x()]);
        let m1 = v1 * v2;

        let v1 = EucVec3f::new([self.1.x(), self.0.z(), self.0.x()]);
        let v2 = EucVec3f::new([self.2.z(), self.2.x(), self.1.z()]);
        let m2 = v1 * v2;

        let s2 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        // Row #3
        let v1 = EucVec3f::new([self.1.x(), self.0.y(), self.0.x()]);
        let v2 = EucVec3f::new([self.2.y(), self.2.x(), self.1.y()]);
        let m1 = v1 * v2;

        let v1 = EucVec3f::new([self.1.y(), self.0.x(), self.0.y()]);
        let v2 = EucVec3f::new([self.2.x(), self.2.y(), self.1.x()]);
        let m2 = v1 * v2;

        let s3 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        Self(
            EucVec3f(s1),
            EucVec3f(s2),
            EucVec3f(s3)
        )
    }
}

impl Mul<EucVec3f> for Mat3f {
    type Output = EucVec3f;

    #[inline(always)]
    fn mul (self, rhs: EucVec3f) -> Self::Output {
        let m1 = EucVec3f::new([self.0.x(), self.1.x(), self.2.x()]) * rhs.x();
        let m2 = EucVec3f::new([self.0.y(), self.1.y(), self.2.y()]) * rhs.y();
        let m3 = EucVec3f::new([self.0.z(), self.1.z(), self.2.z()]) * rhs.z();

        m1 + m2 + m3
    }
}

impl Mul for Mat3f {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let v1 = rhs.0 * self.0.x();
        let v2 = rhs.1 * self.0.y();
        let v3 = rhs.2 * self.0.z();
        let s1 = v1 + v2 + v3;

        let v1 = rhs.0 * self.1.x();
        let v2 = rhs.1 * self.1.y();
        let v3 = rhs.2 * self.1.z();
        let s2 = v1 + v2 + v3;

        let v1 = rhs.0 * self.2.x();
        let v2 = rhs.1 * self.2.y();
        let v3 = rhs.2 * self.2.z();
        let s3 = v1 + v2 + v3;

        Self(s1, s2, s3)
    }
}

impl Into<[f32;9]> for Mat3f {
    #[inline(always)]
    fn into(self) -> [f32;9] {
        unsafe { transmute([Into::<[f32;3]>::into(self.0), self.1.into(), self.2.into()]) }
    }
}

impl Into<Mat3d> for Mat3f {
    #[inline(always)]
    fn into(self) -> Mat3d {
        let c1 = EucVec4f::new([self.xx(), self.xy(), self.xz(), self.yx()]).into();
        let c2 = EucVec4f::new([self.yy(), self.yz(), self.zx(), self.zy()]).into();
        let c3 = self.zz().into();
        Mat3d(c1, c2, c3)
    }
}