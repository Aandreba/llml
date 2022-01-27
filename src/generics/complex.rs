use std::ops::*;
use crate::traits::{ComplexSqrt, Zero};
use crate::vec::{EucVec2f, EucVec2d};

macro_rules! declare {
    ($($name:ident, $og:ident, $ogbig:ident, $ty:ident, $($tag:ident)?),+) => {
        $(  
            #[derive(Default, Clone, Copy, PartialEq, Eq)]
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);
            impl_arith!($name, $ty);

            impl_assign!(
                $name,
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul,
                DivAssign, div_assign, div
            );

            impl_assign!(
                1, $name, $ty,
                AddAssign, add_assign, add,
                SubAssign, sub_assign, sub,
                MulAssign, mul_assign, mul,
                DivAssign, div_assign, div
            );

            impl $name {
                #[inline(always)]
                pub fn new (re: $ty, im: $ty) -> Self {
                    Self($og::new([re, im]))
                }

                #[inline(always)]
                pub fn from_re (re: $ty) -> Self {
                    Self($og::new([re, 0.]))
                }

                #[inline(always)]
                pub fn from_im (im: $ty) -> Self {
                    Self($og::new([0., im]))
                }

                #[inline(always)]
                /// Returns the real & imaginary parts unziped
                pub fn unzip (self) -> ($ty, $ty) {
                    (self.re(), self.im())
                }

                #[inline(always)]
                /// Returns the real part of the complex number
                pub fn re (&self) -> $ty {
                    self.0.x()
                }

                #[inline(always)]
                /// Returns the imaginary part of the complex number
                pub fn im (&self) -> $ty {
                    self.0.y()
                }

                #[inline(always)]
                /// Returns the complex conjugate (```re - im*i```)
                pub fn conj (self) -> Self {
                    Self::new(self.re(), -self.im())
                }

                #[inline(always)]
                /// Returns the radius of the complex number in polar coordinates (```hypot(re, im)```, ```sqrt(re^2 + im^2)```)
                pub fn radius (self) -> $ty {
                    self.re().hypot(self.im())
                }

                /// Returns the angle of the complex number in polar coordinates (```atan2(im, re)```, ```atan(im/re)```)
                #[inline(always)]
                pub fn angle (self) -> $ty {
                    self.im().atan2(self.re())
                }

                #[inline(always)]
                /// Returns the polar coordinates of the complex number as ```(radius, angle)```
                pub fn polar (self) -> ($ty, $ty) {
                    (self.radius(), self.angle())
                }

                /// Computes the **inverse** of the complex number
                #[inline(always)]
                pub fn inv (self) -> Self {
                    Self(self.conj().0 / self.0.dot(self.0))
                }

                /// Calculates the **square root** of the complex number
                #[inline(always)]
                pub fn sqrt (self) -> Self {
                    if self.im().is_zero() {
                        return self.re().sqrtc()
                    }

                    let alpha = self.radius();
                    let res = (($og::new([self.re(), -self.re()]) + alpha) / 2.).sqrt();

                    Self::new(res.x(), self.im().signum() * res.y())
                }

                /// Calculates the **exponential** of the complex number
                #[inline(always)]
                pub fn exp (self) -> Self {
                    self.re().exp() * Self::expi(self.im())
                }

                /// Calculates the **natural logarithm** of the complex number
                #[inline(always)]
                pub fn ln (self) -> Self {
                    let (radius, angle) = self.polar();
                    Self::new(radius.ln(), angle)
                }

                /// Calculates the **power series** of the complex number by an **integer**
                #[inline(always)]
                pub fn powi (self, exp: i32) -> Self {
                    let (radius, angle) = self.polar();
                    radius.powi(exp) * Self::expi(angle * (exp as $ty))
                }

                /// Calculates the **power series** of the complex number by a **real number**
                #[inline(always)]
                pub fn powf (self, exp: $ty) -> Self {
                    let (radius, angle) = self.polar();
                    radius.powf(exp) * Self::expi(angle * exp)
                }

                /// Calculates the **power series** of the complex number by another **complex number**
                #[inline(always)]
                pub fn powc (self, exp: Self) -> Self {
                    let (r, theta) = self.polar();
                    let (x, y) = exp.unzip();

                    let v1 = $og::new(exp.into());
                    let v2 = $og::new([theta, r.ln()]);
                    r.powf(x) * Self::expi(v1.dot(v2)) / (y * theta).exp()
                }

                /// Calculates the **sine** of the complex number
                #[inline(always)]
                pub fn sin (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new([sin_cos.0, sin_cos.1]);
                    let beta = $og::new([self.im().cosh(), self.im().sinh()]);
                    Self(alpha * beta)
                }

                /// Calculates the **cosine** of the complex number
                #[inline(always)]
                pub fn cos (self) -> Self {
                    let sin_cos = self.re().sin_cos();
                    let alpha = $og::new([sin_cos.1, -sin_cos.0]);
                    let beta = $og::new([self.im().cosh(), self.im().sinh()]);
                    Self(alpha * beta)
                }

                /// Calculates the **tangent** of the complex number
                #[inline(always)]
                pub fn tan (self) -> Self {
                    let (re, im) = (2. * self).unzip();
                    let div = re.cos() + im.cosh();
                    Self::new(re.sin(), im.sinh()) / div
                }

                /// Computes the **square root** of the value, returning the complex result
                #[inline(always)]
                pub fn sqrtc (x: $ty) -> Self {
                    if (x >= 0.) { return Self::from_re(x.sqrt()) }
                    Self::from_im((-x).sqrt())
                }

                /// Computes ```exp(self * i)```
                #[inline(always)]
                pub fn expi (x: $ty) -> Self {
                    let sin_cos = x.sin_cos();
                    Self::new(sin_cos.1, sin_cos.0)
                }

                /// Computes ```pow(self, rhs * i)```
                #[inline(always)]
                pub fn powci (x: $ty, rhs: $ty) -> Self {
                    Self::expi(rhs * x.ln())
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

            impl Into<[$ty;2]> for $name {
                #[inline(always)]
                fn into (self) -> [$ty;2] {
                    self.0.into()
                }
            }

            impl Into<$og> for $name {
                #[inline(always)]
                fn into (self) -> $og {
                    self.0
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
    Complxf, EucVec2f, Matf2, f32, ,
    Complxd, EucVec2d, Matd2, f64, q
);

impl Into<Complxd> for Complxf {
    #[inline(always)]
    fn into(self) -> Complxd {
        Complxd(Into::<EucVec2d>::into(self.0))
    }
}

impl Into<Complxf> for Complxd {
    #[inline(always)]
    fn into(self) -> Complxf {
        Complxf(self.0.into())
    }
}