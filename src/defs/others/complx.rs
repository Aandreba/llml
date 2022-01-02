import_derives!();

use std::{ops::{Neg, Mul, Div}, intrinsics::transmute};
use crate::vec::EucVec2;

use super::{Exp, SinCos, Hypot, Atan2, Ln, Zero, Sqrt, ComplxSqrt, Two, Signum, Arith, Sin, Cos, Sinh, Cosh, SinhCosh, Tan};

pub type Complxf = Complx<f32>;
pub type Complxd = Complx<f64>;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct Complx<T> {
    pub re: T,
    pub im: T
}

impl<T> Complx<T>  {
    pub fn new (re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn of_re (re: T) -> Self where T: Zero {
        Self { re, im: T::zero() }
    }

    pub fn of_im (im: T) -> Self where T: Zero {
        Self { re: T::zero(), im }
    }

    #[inline(always)]
    pub fn conj (self) -> Self where T: Neg<Output = T> {
        Self::new(self.re, -self.im)
    }

    #[inline(always)]
    pub fn radius (self) -> T where T: Hypot {
        self.re.hypot(self.im)
    }

    #[inline(always)]
    pub fn angle (self) -> T where T: Atan2 {
        self.im.atan2(self.re)
    }

    /// Returns the complex value in polar coordinates
    #[inline]
    pub fn polar (self) -> Polar<T> where T: Hypot + Atan2 + Clone {
        Polar::new(self.clone().radius(), self.angle())
    }

    #[inline]
    pub fn expi (x: T) -> Complx<T> where T: SinCos {
        let sin_cos = x.sin_cos();
        Complx::new(sin_cos.1, sin_cos.0)
    }
}

impl<T: ComplxSqrt + Zero + PartialEq + Hypot + Two + Copy + Signum + Arith> Sqrt for Complx<T> {
    fn sqrt(self) -> Self {
        if self.im.is_zero() {
            return self.re.sqrtc()
        }

        let alpha = self.radius();
        Self::new((alpha + self.re) / T::two(), self.im.signum() * (alpha - self.re) / T::two())
    }
}

impl<T: Exp + SinCos> Exp for Complx<T> where Self: Mul<T, Output = Self> {
    fn exp (self) -> Self {
        Self::expi(self.im) * self.re.exp()
    }
}

impl<T: Hypot + Atan2 + Ln + Clone> Ln for Complx<T> {
    fn ln (self) -> Self {
        let polar = self.polar();
        Self::new(polar.radius.ln(), polar.angle)
    }
}

impl<T: SinCos + SinhCosh + Copy> Sin for Complx<T> where EucVec2<T>: Mul<Output = EucVec2<T>>  {
    fn sin (self) -> Self {
        let sin_cos = self.re.sin_cos();
        let sinh_cosh = self.im.sinh_cosh();

        let mul = EucVec2::new(sin_cos.0, sinh_cosh.1) * EucVec2::new(sin_cos.1, sinh_cosh.0);
        Self::new(mul.x, mul.y)
    }
}

impl<T: SinCos + SinhCosh + Neg<Output = T> + Copy> Cos for Complx<T> where EucVec2<T>: Mul<Output = EucVec2<T>>  {
    fn cos (self) -> Self {
        let sin_cos = self.re.sin_cos();
        let sinh_cosh = self.im.sinh_cosh();
        
        let mul = EucVec2::new(sin_cos.1, sinh_cosh.1) * EucVec2::new(-sin_cos.0, sinh_cosh.0);
        Self::new(mul.x, mul.y)
    }
}

impl<T> SinCos for Complx<T> where Self: Sin + Cos + Clone {
    fn sin_cos (self) -> (Self, Self) {
        (self.clone().sin(), self.cos())
    }
}

impl<T> Tan for Complx<T> where Self: SinCos + Div<Output = Self>  {
    fn tan (self) -> Self {
        let sin_cos = self.sin_cos();
        sin_cos.0 / sin_cos.1
    }
}

impl<T: Zero> Zero for Complx<T> {
    fn zero () -> Self {
        Self::new(T::zero(), T::zero())
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
// TODO RAND
pub struct Polar<T> {
    pub radius: T,
    pub angle: T
}

impl<T> Polar<T> {
    pub fn new (radius: T, angle: T) -> Self {
        Self { radius, angle }
    }
}