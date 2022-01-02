use std::{ops::{Neg, Sub, Mul, Div, Add}, process::Output};

use super::Complx;

// MACROS
macro_rules! impl_const {
    ($trait:ident, $fn:ident, $val:expr, $($tg:ident),+) => {
        $(
            impl $trait for $tg {
                fn $fn () -> Self {
                    $val
                }
            }
        )*
    }
}

macro_rules! impl_const_i {
    ($trait:ident, $fn:ident, $val:expr) => {
        impl_const!($trait, $fn, $val, u8, u16, u32, u64, u128, usize);
        impl_const!($trait, $fn, $val, i8, i16, i32, i64, i128, isize);
    };
}

macro_rules! impl_const_f {
    ($trait:ident, $fn:ident, $val:expr) => {
        impl_const!($trait, $fn, $val, f32, f64);
    };
}

macro_rules! declare_simple {
    ($($name:ident, $fn:ident),+) => {
        $(
            pub trait $name {
                fn $fn (self) -> Self;
            }
        )*
    };
}

macro_rules! declare_double {
    ($($name:ident, $fn:ident),+) => {
        $(
            pub trait $name {
                fn $fn (self, rhs: Self) -> Self;
            }
        )*
    };
}

macro_rules! impl_simple {
    ($trait:ident, $fn:ident, $($tg:ident),+) => {
        $(
            impl $trait for $tg {
                #[inline]
                #[doc=concat!("Alias for [primitive](https://doc.rust-lang.org/stable/std/primitive.", stringify!($tg), ".html#method.", stringify!($fn), ")")]
                fn $fn (self) -> Self {
                    $tg::$fn(self)
                }
            }
        )*
    };
}

macro_rules! impl_double {
    ($trait:ident, $fn:ident, $($tg:ident),+) => {
        $(
            impl $trait for $tg {
                #[inline]
                #[doc=concat!("Alias for [primitive](https://doc.rust-lang.org/stable/std/primitive.", stringify!($tg), ".html#method.", stringify!($fn), ")")]
                fn $fn (self, rhs: Self) -> Self {
                    $tg::$fn(self, rhs)
                }
            }
        )*
    };
}

macro_rules! impl_simple_i {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_simple!($trait, $fn, i8, i16, i32, i64, i128, isize);)*
    }
}

macro_rules! impl_simple_u {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_simple!($trait, $fn, u8, u16, u32, u64, u128, usize);)*
    }
}

macro_rules! impl_simple_f {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_simple!($trait, $fn, f32, f64);)*
    }
}

macro_rules! impl_simple_a {
    ($($trait:ident, $fn:ident),+) => {
        $(
            impl_simple_u!($trait, $fn);
            impl_simple_i!($trait, $fn);
            impl_simple_f!($trait, $fn);
        )*
    }
}

macro_rules! impl_double_i {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_double!($trait, $fn, i8, i16, i32, i64, i128, isize);)*
    }
}

macro_rules! impl_double_u {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_double!($trait, $fn, u8, u16, u32, u64, u128, usize);)*
    }
}

macro_rules! impl_double_f {
    ($($trait:ident, $fn:ident),+) => {
        $(impl_double!($trait, $fn, f32, f64);)*
    }
}

macro_rules! impl_double_a {
    ($($trait:ident, $fn:ident),+) => {
        $(
            impl_double_u!($trait, $fn);
            impl_double_i!($trait, $fn);
            impl_double_f!($trait, $fn);
        )*
    }
}

macro_rules! declare_and_impl_simple_i {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_simple!($name, $fn);
            impl_simple_i!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_simple_u {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_simple!($name, $fn);
            impl_simple_u!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_simple_f {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_simple!($name, $fn);
            impl_simple_f!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_simple_a {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_simple!($name, $fn);
            impl_simple_a!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_double_i {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_double!($name, $fn);
            impl_double_i!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_double_u {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_double!($name, $fn);
            impl_double_u!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_double_f {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_double!($name, $fn);
            impl_double_f!($name, $fn);
        )*
    }
}

macro_rules! declare_and_impl_double_a {
    ($($name:ident, $fn:ident),+) => {
        $(
            declare_double!($name, $fn);
            impl_double_a!($name, $fn);
        )*
    }
}

// DECLARATIONS
declare_and_impl_simple_f!(
    Abs, abs,
    Signum, signum,

    Round, round,
    Sqrt, sqrt,

    Exp, exp,
    Ln, ln,

    Sin, sin,
    Cos, cos,
    Tan, tan,

    Asin, asin,
    Acos, acos,
    Atan, atan,

    Sinh, sinh,
    Cosh, cosh,
    Tanh, tanh
);

impl_simple_i!(
    Abs, abs,
    Signum, signum
);

declare_and_impl_double_f!(
    Atan2, atan2,
    Copysign, copysign
);

// ARITHMETIC
pub trait Arith: Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {}
impl<T> Arith for T where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> {}

// HYPOTENUSE
pub trait Hypot: Sqrt {
    fn hypot (self, rhs: Self) -> Self;
}

impl_double_f!(Hypot, hypot);

// SIN COS
pub trait SinCos: Sin + Cos where Self: Sized {
    fn sin_cos (self) -> (Self, Self);
}

impl SinCos for f32 {
    #[inline]
    fn sin_cos (self) -> (Self, Self) {
        f32::sin_cos(self)
    }
}

impl SinCos for f64 {
    #[inline]
    fn sin_cos (self) -> (Self, Self) {
        f64::sin_cos(self)
    }
}

// SINH COSH
pub trait SinhCosh: Sized + Sinh + Cosh {
    fn sinh_cosh (self) -> (Self, Self);
}

impl<T> SinhCosh for T where T: Sized + Sinh + Cosh + Clone {
    fn sinh_cosh (self) -> (Self, Self) {
        (self.clone().sinh(), self.cosh())
    }
}

// COMPLEX SQRT
pub trait ComplxSqrt where Self: Sized {
    fn sqrtc (self) -> Complx<Self>;
}

impl<T> ComplxSqrt for T where T: Sqrt + Zero + PartialOrd + Neg<Output = T> {
    fn sqrtc (self) -> Complx<Self> {
        if self >= Self::zero() {
            return Complx::of_re(self.sqrt())
        }

        Complx::of_im(-self.sqrt())
    }
}

// POWER
macro_rules! impl_pow {
    () => {
        impl_pow_into!(u8, u16, u32);
        impl_pow_try!(i8, i16, i32, u64, i64, u128, i128, usize, isize);
        impl_pow_f!(f32, f64);
    };
}

macro_rules! impl_pow_f {
    () => {
        impl_pow_f!(f32, f64);
    };

    ($($ty:ident),+) => {
        $(
            impl Pow for $ty {
                fn pow (self, rhs: Self) -> Self {
                    $ty::powf(self, rhs)
                }
            }
        )*
    }
}

macro_rules! impl_pow_into {
    ($($ty:ident),+) => {
        $(
            impl Pow for $ty {
                fn pow (self, rhs: Self) -> Self {
                    $ty::pow(self, <Self as Into<u32>>::into(rhs))
                }
            }
        )*
    };
}

macro_rules! impl_pow_try {
    ($($ty:ident),+) => {
        $(
            impl Pow for $ty {
                fn pow (self, rhs: Self) -> Self {
                    $ty::pow(self, <Self as TryInto<u32>>::try_into(rhs).unwrap())
                }
            }
        )*
    };
}

pub trait Pow {
    fn pow (self, rhs: Self) -> Self;
}

impl_pow!();

// CONSTANTS
pub trait Zero {
    fn zero () -> Self;
    fn is_zero (self) -> bool where Self: Sized + PartialEq {
        self == Self::zero()
    }
}

pub trait One {
    fn one () -> Self;
    fn is_one (self) -> bool where Self: Sized + PartialEq {
        self == Self::one()
    }
}

pub trait Two {
    fn two () -> Self;
    fn is_two (self) -> bool where Self: Sized + PartialEq {
        self == Self::two()
    }
}

impl_const_i!(Zero, zero, 0);
impl_const_f!(Zero, zero, 0.);

impl_const_i!(One, one, 1);
impl_const_f!(One, one, 1.);

impl_const_i!(Two, two, 1);
impl_const_f!(Two, two, 1.);