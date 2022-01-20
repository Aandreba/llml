use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::traits::{ComplexSqrt, Zero};
use crate::vec::{EucVecf2, EucVecd2};
use crate::polar::Polar;

macro_rules! declare {
    ($($name:ident, $og:ident, $ogbig:ident, $ty:ident, $($tag:ident)?),+) => {
        $(  
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);
            impl_arith!($name, $ty);

            impl $name {
                #[inline(always)]
                pub fn new (re: $ty, im: $ty) -> Self {
                    Self($og::new([re, im]))
                }

                #[inline(always)]
                pub fn re (&self) -> $ty {
                    self.0.x()
                }

                #[inline(always)]
                pub fn im (&self) -> $ty {
                    self.0.y()
                }

                #[inline(always)]
                pub fn conj (self) -> Self {
                    Self::new(self.re(), -self.im())
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
                    let res = (($og::new([self.re(), -self.re()]) + alpha) / 2.).sqrt();

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
                    todo!();
                    //Self::exp(exp * self.ln()) // TODO
                }

                #[inline(always)]
                pub fn sin (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new([sin_cos.0, sin_cos.1]);
                    let beta = $og::new([self.im().cosh(), self.im().sinh()]);
                    Self(alpha * beta)
                }

                #[inline(always)]
                pub fn cos (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new([sin_cos.1, -sin_cos.0]);
                    let beta = $og::new([self.im().cosh(), self.im().sinh()]);
                    Self(alpha * beta)
                }

                #[inline(always)]
                pub fn tan (self) -> Self {
                    let two = 2. * self;
                    let re = two.re();
                    let im = two.im();

                    let div = re.cos() + im.cosh();
                    Self::new(re.sin(), im.sinh()) / div
                }

                #[inline(always)]
                pub fn expi (x: $ty) -> Self {
                    let sin_cos = x.sin_cos();
                    Self::new(sin_cos.1, sin_cos.0)
                }
            }

            impl Neg for $name {
                type Output = Self;

                #[inline(always)]
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

impl Into<Complxd> for Complxf {
    #[inline(always)]
    fn into(self) -> Complxd {
        Complxd(Into::<EucVecd2>::into(self.0))
    }
}

impl Into<Complxf> for Complxd {
    #[inline(always)]
    fn into(self) -> Complxf {
        Complxf(self.0.into())
    }
}