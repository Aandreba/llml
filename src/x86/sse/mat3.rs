x86_use!();
use crate::{traits::Zero, x86::{vec3::EucVecf3, vec2::EucVecf2, vec4::EucVecf4, _mm_low_high_ps, _mm_sum_ps, _mm_combine_ps}, mat::Matd3};
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
            impl $trait for Matf3 {
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
            impl $trait<f32> for Matf3 {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: f32) -> Self::Output {
                    unsafe {
                        let rhs = _mm_set1_ps(rhs);
                        let x = concat_idents!(_mm_, $fun, _ps)(self.0.0, rhs);
                        let y = concat_idents!(_mm_, $fun, _ps)(self.1.0, rhs);
                        let z = concat_idents!(_mm_, $fun, _ps)(self.2.0, rhs);
                        Self(EucVecf3(x), EucVecf3(y), EucVecf3(z))
                    }
                }
            }

            impl $trait<Matf3> for f32 {
                type Output = Matf3;

                #[inline(always)]
                fn $fun (self, rhs: Matf3) -> Self::Output {
                    unsafe {
                        let sf = _mm_set1_ps(self);
                        let x = concat_idents!(_mm_, $fun, _ps)(sf, rhs.0.0);
                        let y = concat_idents!(_mm_, $fun, _ps)(sf, rhs.1.0);
                        let z = concat_idents!(_mm_, $fun, _ps)(sf, rhs.2.0);
                        Matf3(EucVecf3(x), EucVecf3(y), EucVecf3(z))
                    }
                }
            }
        )*
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct Matf3 (pub(crate) EucVecf3, pub(crate) EucVecf3, pub(crate) EucVecf3);
impl_matf3!();

impl Neg for Matf3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Matf3 {
    #[inline]
    pub fn new (a: [f32;9]) -> Self {
        Matf3(
            EucVecf3::new([a[0], a[1], a[2]]),
            EucVecf3::new([a[3], a[4], a[5]]),
            EucVecf3::new([a[6], a[7], a[8]])
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
    pub fn x (&self) -> EucVecf3 {
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
    pub fn y (&self) -> EucVecf3 {
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
    pub fn z (&self) -> EucVecf3 {
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
        EucVecf3::new([self.0.x(), self.1.y(), self.2.z()]).sum()
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        // Negation
        let neg = -self.0;

        // Subdets 1 & 2
        let v0 = EucVecf4::new([self.0.x(), neg.y(), neg.x(), self.0.y()]);
        let v1 = EucVecf4::new([self.1.y(), self.1.x(), self.1.z(), self.1.z()]);
        let v2 = EucVecf4::new([self.2.z(), self.2.z(), self.2.y(), self.2.x()]);
        let m1 = v0 * v1 * v2;

        let s1;
        unsafe {
            let (v1, v2) = _mm_low_high_ps(m1.0);
            s1 = _mm_add_ps(v1, v2);
        }

        // Subdet 3
        let v5 = EucVecf2::new([self.0.z(), neg.z()]);
        let v6 = EucVecf2::new([self.1.x(), self.1.y()]);
        let v7 = EucVecf2::new([self.2.y(), self.2.x()]);
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
        let v1 = EucVecf3::new([self.1.y(), self.0.z(), self.0.y()]);
        let v2 = EucVecf3::new([self.2.z(), self.2.y(), self.1.z()]);
        let m1 = v1 * v2;

        let v1 = EucVecf3::new([self.1.z(), self.0.y(), self.0.z()]);
        let v2 = EucVecf3::new([self.2.y(), self.2.z(), self.1.y()]);
        let m2 = v1 * v2;

        let s1 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        // Row #2
        let v1 = EucVecf3::new([self.1.z(), self.0.x(), self.0.z()]);
        let v2 = EucVecf3::new([self.2.x(), self.2.z(), self.1.x()]);
        let m1 = v1 * v2;

        let v1 = EucVecf3::new([self.1.x(), self.0.z(), self.0.x()]);
        let v2 = EucVecf3::new([self.2.z(), self.2.x(), self.1.z()]);
        let m2 = v1 * v2;

        let s2 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        // Row #3
        let v1 = EucVecf3::new([self.1.x(), self.0.y(), self.0.x()]);
        let v2 = EucVecf3::new([self.2.y(), self.2.x(), self.1.y()]);
        let m1 = v1 * v2;

        let v1 = EucVecf3::new([self.1.y(), self.0.x(), self.0.y()]);
        let v2 = EucVecf3::new([self.2.x(), self.2.y(), self.1.x()]);
        let m2 = v1 * v2;

        let s3 = _mm_div_ps(_mm_sub_ps(m1.0, m2.0), det);

        Self(
            EucVecf3(s1),
            EucVecf3(s2),
            EucVecf3(s3)
        )
    }
}

impl Mul<EucVecf3> for Matf3 {
    type Output = EucVecf3;

    #[inline(always)]
    fn mul (self, rhs: EucVecf3) -> Self::Output {
        let m1 = EucVecf3::new([self.0.x(), self.1.x(), self.2.x()]) * rhs.x();
        let m2 = EucVecf3::new([self.0.y(), self.1.y(), self.2.y()]) * rhs.y();
        let m3 = EucVecf3::new([self.0.z(), self.1.z(), self.2.z()]) * rhs.z();

        m1 + m2 + m3
    }
}

impl Mul for Matf3 {
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

impl Into<[f32;9]> for Matf3 {
    #[inline(always)]
    fn into(self) -> [f32;9] {
        unsafe { transmute([Into::<[f32;3]>::into(self.0), self.1.into(), self.2.into()]) }
    }
}

impl Into<Matd3> for Matf3 {
    #[inline(always)]
    fn into(self) -> Matd3 {
        let c1 = EucVecf4::new([self.xx(), self.xy(), self.xz(), self.yx()]).into();
        let c2 = EucVecf4::new([self.yy(), self.yz(), self.zx(), self.zy()]).into();
        let c3 = self.zz().into();
        Matd3(c1, c2, c3)
    }
}