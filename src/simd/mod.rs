use std::simd::{Simd, LaneCount, SupportedLaneCount, SimdElement};

macro_rules! define_norm {
    (EucVec2, $target:ident) => {
        #[inline(always)]
        pub fn norm (self) -> $target where $target: Hypot {
            self.x.hypot(self.y)
        }

        #[inline(always)]
        pub fn unit (self) -> Self where $target: Hypot {
            self / self.norm()
        }
    };

    ($base:ident, $target:ident) => {
        #[inline(always)]
        pub fn norm (self) -> $target where $target: Sqrt {
            self.norm2().sqrt()
        }

        #[inline(always)]
        pub fn unit (self) -> Self where $target: Sqrt {
            self / self.norm()
        }
    }
}

macro_rules! define_neg {
    ($base:ident) => {
        define_neg!(
            $base,
            i8, i16, i32, i64, isize,
            f32, f64
        );
    };

    ($base:ident, $($ty:ident),+) => {
        $(
            impl Neg for $base<$ty> {
                type Output = Self;

                fn neg (self) -> Self {
                    unsafe { Self::from_simd(-self.into_simd()) }
                }
            }
        )*
    }
}

macro_rules! simd_map {
    ($base:ident) => {
        simd_map!(
            $base,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize,
            f32, f64
        );

        define_neg!($base);
    };

    ($base:ident, $($target:ident),+) => {
        use crate::simd::IntoSimd;
        use std::ops::{Add, Sub, Mul, Div, Neg};
        use crate::others::{Hypot, Sqrt};

        $(
            simd_map!(
                $base, $target,
                Add, add, +,
                Sub, sub, -,
                Mul, mul, *,
                Div, div, /
            );

            impl $base<$target> {
                #[inline(always)]
                pub fn sum (self) -> $target {
                    unsafe { self.into_simd().horizontal_sum() }
                }

                #[inline(always)]
                pub fn dot (self, rhs: $base<$target>) -> $target {
                    unsafe { (self.into_simd() * rhs.into_simd()).horizontal_sum() }
                }

                /// Returns ```norm(self)^2```
                #[inline(always)]
                pub fn norm2 (self) -> $target {
                    self.dot(self)
                }

                define_norm!($base, $target);
            }
        )*
    };

    ($t1:ident, $t2:ident, $($trait:ident, $fn:ident, $sy:tt),+) => {
        $(
            impl $trait for $t1<$t2> {
                type Output = Self;

                #[inline(always)]
                fn $fn (self, rhs: Self) -> Self::Output {
                    unsafe { Self::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }

            impl $trait<$t2> for $t1<$t2> {
                type Output = Self;

                #[inline(always)]
                fn $fn (self, rhs: $t2) -> Self::Output {
                    unsafe { Self::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }

            impl $trait<$t1<$t2>> for $t2 {
                type Output = $t1<$t2>;

                #[inline(always)]
                fn $fn (self, rhs: $t1<$t2>) -> Self::Output {
                    unsafe { $t1::<$t2>::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }
        )*
    };
}

macro_rules! simd_mat_map {
    ($t1:ident) => {
        use crate::simd::IntoSimd;
        use std::ops::{Add, Sub, Mul, Div};

        simd_mat_map!(
            $t1,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize,
            f32, f64
        );
    };

    ($t1:ident, $($t2:ident),+) => {
        $(
            simd_mat_map!(
                $t1, $t2,
                Add, add, +,
                Sub, sub, -
            );

            simd_mat_scalar!(
                $t1, $t2,
                Mul, mul, *,
                Div, div, /
            );
        )*
    };

    ($t1:ident, $t2:ident, $($trait:ident, $fn:ident, $sy:tt),+) => {
        $(
            impl $trait for $t1<$t2> {
                type Output = Self;

                #[inline(always)]
                fn $fn (self, rhs: Self) -> Self::Output {
                    unsafe { Self::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }

            simd_mat_scalar!($t1, $t2, $trait, $fn, $sy);
        )*
    };
}

macro_rules! simd_mat_scalar {
    ($t1:ident, $t2:ident, $($trait:ident, $fn:ident, $sy:tt),+) => {
        $(
            impl $trait<$t2> for $t1<$t2> {
                type Output = Self;

                #[inline(always)]
                fn $fn (self, rhs: $t2) -> Self::Output {
                    unsafe { Self::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }

            impl $trait<$t1<$t2>> for $t2 {
                type Output = $t1<$t2>;

                #[inline(always)]
                fn $fn (self, rhs: $t1<$t2>) -> Self::Output {
                    unsafe { $t1::<$t2>::from_simd(self.into_simd() $sy rhs.into_simd()) }
                }
            }
        )*
    };
}

unsafe trait IntoSimd: SimdElement {
    fn into_simd<const N: usize> (self) -> Simd<Self, N> where LaneCount<N>: SupportedLaneCount;
}

unsafe impl<T> IntoSimd for T where T: SimdElement {
    #[inline(always)]
    fn into_simd<const N: usize> (self) -> Simd<Self, N> where LaneCount<N>: SupportedLaneCount {
        Simd::from_array([self;N])
    }
}

flat_mod!(vec2, vec3, vec4);
flat_mod!(mat2, mat3, mat4);