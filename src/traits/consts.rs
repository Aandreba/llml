macro_rules! def_const {
    ($($name:ident, $fun1:ident, $fun2:ident, $int:literal, $float:literal),+) => {
        $(
            pub trait $name {
                fn $fun1 () -> Self;
                fn $fun2 (self) -> bool;
            }

            impl_const_int!($name, $fun1, $fun2, $int, u8, u16, u32, u64, u128, usize);
            impl_const_int!($name, $fun1, $fun2, $int, i8, i16, i32, i64, i128, isize);
            impl_const_float!($name, $fun1, $fun2, $float, f32, f64);
        )*
    };
}

macro_rules! impl_const_int {
    ($name:ident, $fun1:ident, $fun2:ident, $int:literal, $($ty:ident),+) => {
        $(
            impl $name for $ty {
                #[inline(always)]
                fn $fun1 () -> Self {
                    $int
                }

                #[inline(always)]
                fn $fun2 (self) -> bool {
                    self == $int
                }
            }
        )*
    };
}

macro_rules! impl_const_float {
    ($name:ident, $fun1:ident, $fun2:ident, 0., $($ty:ident),+) => {
        $(
            impl $name for $ty {
                #[inline(always)]
                fn $fun1 () -> Self {
                    $float
                }

                #[inline(always)]
                fn $fun2 (self) -> bool {
                    self.abs() <= $ty::EPSILON
                }
            }
        )*
    };

    ($name:ident, $fun1:ident, $fun2:ident, $float:literal, $($ty:ident),+) => {
        $(
            impl $name for $ty {
                #[inline(always)]
                fn $fun1 () -> Self {
                    $float
                }

                #[inline(always)]
                fn $fun2 (self) -> bool {
                    (self - $float).abs() <= $ty::EPSILON
                }
            }
        )*
    };
}

def_const!(
    Zero, zero, is_zero, 0, 0.,
    One, one, is_one, 1, 1.,
    Two, two, is_two, 2, 2.
);