arm_use!();
use crate::EucVecf3;
use std::ops::{Add, Sub, Mul, Div, Neg};

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
                        let rhs = vld1q_dup_f32(&rhs);
                        let x = concat_idents!(v, $fun, q_f32)(self.0.0, rhs);
                        let y = concat_idents!(v, $fun, q_f32)(self.1.0, rhs);
                        let z = concat_idents!(v, $fun, q_f32)(self.2.0, rhs);
                        Self(EucVecf3(x), EucVecf3(y), EucVecf3(z))
                    }
                }
            }

            impl $trait<Matf3> for f32 {
                type Output = Matf3;

                #[inline(always)]
                fn $fun (self, rhs: Matf3) -> Self::Output {
                    unsafe {
                        let sf = vld1q_dup_f32(&self);
                        let x = concat_idents!(v, $fun, q_f32)(sf, rhs.0.0);
                        let y = concat_idents!(v, $fun, q_f32)(sf, rhs.1.0);
                        let z = concat_idents!(v, $fun, q_f32)(sf, rhs.2.0);
                        Matf3(EucVecf3(x), EucVecf3(y), EucVecf3(z))
                    }
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct Matf3 (pub(crate) EucVecf3, pub(crate) EucVecf3, pub(crate) EucVecf3);
impl_matf3!();

impl Matf3 {
    #[inline]
    pub fn new (a: [f32;9]) -> Self {
        Matf3(
            EucVecf3::new(a[0], a[1], a[2]),
            EucVecf3::new(a[3], a[4], a[5]),
            EucVecf3::new(a[6], a[7], a[8])
        )
    }
}

impl Mul<EucVecf3> for Matf3 {
    type Output = EucVecf3;

    #[inline(always)]
    fn mul (self, rhs: EucVecf3) -> Self::Output {
        let m1 = EucVecf3::new(self.0.x(), self.1.x(), self.2.x()) * rhs.x();
        let m2 = EucVecf3::new(self.0.y(), self.1.y(), self.2.y()) * rhs.y();
        let m3 = EucVecf3::new(self.0.z(), self.1.z(), self.2.z()) * rhs.z();

        m1 + m2 + m3
    }
}

impl Mul for Matf3 {
    type Output = Self;

    //#[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let sv1 = EucVecf3::new(self.0.x(), self.1.x(), self.2.x());
        let sv2 = EucVecf3::new(self.0.y(), self.1.y(), self.2.y());
        let sv3 = EucVecf3::new(self.0.z(), self.1.z(), self.2.z());

        let v1 = sv1 * rhs.0.x();
        let v2 = sv2 * rhs.0.y();
        let v3 = sv3 * rhs.0.z();
        let s1 = v1 + v2 + v3;

        let v1 = sv1 * rhs.1.x();
        let v2 = sv2 * rhs.1.y();
        let v3 = sv3 * rhs.1.z();
        let s2 = v1 + v2 + v3;

        let v1 = sv1 * rhs.2.x();
        let v2 = sv2 * rhs.2.y();
        let v3 = sv3 * rhs.2.z();
        let s3 = v1 + v2 + v3;

        todo!()
        //Self(s1, s2, s3)
    }
}