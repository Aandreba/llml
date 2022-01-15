use std::intrinsics::transmute;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::traits::{Complx, ComplexSqrt, Zero};
use crate::{EucVecf2, EucVecd2, EucVecf4, EucVecd4};
use crate::Polar;

arm_use!();

macro_rules! declare {
    ($($name:ident, $og:ident, $ogbig:ident, $ty:ident, $($tag:ident)?),+) => {
        $(  
            #[derive(Debug, Clone, Copy)]
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);
            impl_arith!($name, $ty);

            impl $name {
                #[inline(always)]
                pub fn conj (self) -> Self {
                    unsafe {
                        Self($og(concat_idents!(vset, $($tag,)? _lane_, $ty)(-concat_idents!(vget, $($tag,)? _lane_, $ty)(self.0.0, 1), self.0.0, 1)))
                    }
                }

                #[inline(always)]
                pub fn radius (self) -> $ty {
                    self.re().hypot(self.im())
                }

                #[inline(always)]
                pub fn angle (self) -> $ty {
                    self.im().atan2(self.re())
                }

                #[inline(always)]
                pub fn polar (self) -> Polar<$ty> {
                    Polar::new(self.radius(), self.angle())
                }

                #[inline(always)]
                pub fn inv (self) -> Self {
                    Self(self.conj().0 / self.0.dot(self.0))
                }

                #[inline(always)]
                pub fn sqrt (self) -> Self {
                    if self.im().is_zero() {
                        return self.re().sqrtc()
                    }

                    let alpha = self.radius();
                    let res = (($og::new(self.re(), -self.re()) + alpha) / 2.).sqrt();

                    Self::new(res.x(), self.im().signum() * res.y())
                }

                #[inline(always)]
                pub fn exp (self) -> Self {
                    self.re().exp() * Self::expi(self.im())
                }

                #[inline(always)]
                pub fn ln (self) -> Self {
                    let polar = self.polar();
                    Self::new(polar.radius.ln(), polar.angle)
                }

                #[inline(always)]
                pub fn powi (self, exp: i32) -> Self {
                    let polar = self.polar();
                    polar.radius.powi(exp) * Self::expi(polar.angle * (exp as $ty))
                }

                #[inline(always)]
                pub fn powf (self, exp: $ty) -> Self {
                    let polar = self.polar();
                    polar.radius.powf(exp) * Self::expi(polar.angle * (exp as $ty))
                }

                #[inline(always)]
                pub fn powc (self, exp: Self) -> Self {
                    Self::exp(exp * self.ln()) // TODO
                }

                #[inline(always)]
                pub fn sin (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new(sin_cos.0, sin_cos.1);
                    let beta = $og::new(self.im().cosh(), self.im().sinh());
                    Self(alpha * beta)
                }

                #[inline(always)]
                pub fn cos (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new(sin_cos.1, -sin_cos.0);
                    let beta = $og::new(self.im().cosh(), self.im().sinh());
                    Self(alpha * beta)
                }

                #[inline(always)]
                pub fn expi (x: $ty) -> Self {
                    let sin_cos = x.sin_cos();
                    Self::new(sin_cos.1, sin_cos.0)
                }
            }

            impl Neg for $name {
                type Output = Self;

                fn neg (self) -> Self::Output {
                    Self(-self.0)
                }
            }

            impl Div for $name {
                type Output = Self;
                
                #[inline(always)]
                fn div (self, rhs: Self) -> Self::Output {
                    self * rhs.inv()
                }
            }

            impl Complx<$ty> for $name {
                #[inline(always)]
                fn new (re: $ty, im: $ty) -> Self {
                    Self($og::new(re, im))
                }

                #[inline(always)]
                fn re (&self) -> $ty {
                    self.0.x()
                }

                #[inline(always)]
                fn im (&self) -> $ty {
                    self.0.y()
                }
            }
        )*
    };
}

macro_rules! impl_arith {
    ($($target:ident, $ty:ident),+) => {
        $(
            impl_arith!(
                $target, $ty,
                Add, add, +,
                Sub, sub, -
            );

            impl_arith_scal!(
                $target, $ty,
                Mul, mul, *,
                Div, div, /
            );
        )*
    };

    ($target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0 $sy rhs.0)
                }
            }

            impl_arith_scal!($target, $ty, $trait, $fun, $sy);
        )*
    };  
}

macro_rules! impl_arith_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0 $sy rhs)
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self $sy rhs.0)
                }
            }
        )*
    }; 
}

declare!(
    Complxf, EucVecf2, Matf2, f32, ,
    Complxd, EucVecd2, Matd2, f64, q
);

impl Mul for Complxf {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let v1 = vcombine_f32(vtrn1_f32(self.0.0, self.0.0), vtrn2_f32(self.0.0, self.0.0));
            let v2 = vcombine_f32(rhs.0.0, transmute([-rhs.im(), rhs.re()]));
            let m1 = vmulq_f32(v1, v2);

            Self(EucVecf2(vadd_f32(vget_low_f32(m1), vget_high_f32(m1))))
        }
    }
}

impl Mul for Complxd {
    type Output = Self;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let m1 = vmulq_f64(vtrn1q_f64(self.0.0, self.0.0), rhs.0.0);
            let m2 = vmulq_f64(vtrn2q_f64(self.0.0, self.0.0), transmute([-rhs.im(), rhs.re()]));
            Self(EucVecd2(vaddq_f64(m1, m2)))
        }
    }
}

impl Complxf {
    #[inline(always)]
    pub fn tan (self) -> Self {
        let sin_cos = self.re().sin_cos();
        let beta = EucVecf2::new(self.im().cosh(), self.im().sinh());

        unsafe {
            let alpha = EucVecf4::new(sin_cos.0, sin_cos.1, sin_cos.1, -sin_cos.0);
            let sin_cos = vmulq_f32(alpha.0, vcombine_f32(beta.0, beta.0));
            
            let sin = Complxf(EucVecf2(vget_low_f32(sin_cos)));
            let cos = Complxf(EucVecf2(vget_high_f32(sin_cos)));
            sin / cos
        }
    }
}

impl Complxd {
    #[inline(always)]
    pub fn tan (self) -> Self {
        let sin_cos = self.re().sin_cos();
        let beta = EucVecd2::new(self.im().cosh(), self.im().sinh());

        let alpha = EucVecd4::new(sin_cos.0, sin_cos.1, sin_cos.1, -sin_cos.0);
        let sin_cos = alpha * EucVecd4(beta, beta);
        Complxd(sin_cos.0) / Complxd(sin_cos.1)
    }
}

impl Into<Complxd> for Complxf {
    #[inline(always)]
    fn into(self) -> Complxd {
        Complxd(self.0.into())
    }
}

impl Into<Complxf> for Complxd {
    #[inline(always)]
    fn into(self) -> Complxf {
        Complxf(self.0.into())
    }
}