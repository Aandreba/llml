use num::{Complex, Num, Float};
use std::concat_idents;

macro_rules! int_consts {
    ($($t:ty),*) => {
        $(
            impl Consts for $t {
                fn two () -> Self {
                    2
                }

                fn three () -> Self {
                    3
                }

                fn four () -> Self {
                    4
                }

                fn five () -> Self {
                    5
                }

                fn six () -> Self {
                    6
                }

                fn seven () -> Self {
                    7
                }

                fn eight () -> Self {
                    8
                }

                fn nine () -> Self {
                    9
                }

                fn twenty_seven () -> Self {
                    27
                }
            }
        )*
    };
}

macro_rules! float_consts {
    ($($t:ty),*) => {
        $(
            impl Consts for $t {
                fn two () -> Self {
                    2.
                }

                fn three () -> Self {
                    3.
                }

                fn four () -> Self {
                    4.
                }

                fn five () -> Self {
                    5.
                }

                fn six () -> Self {
                    6.
                }

                fn seven () -> Self {
                    7.
                }

                fn eight () -> Self {
                    8.
                }

                fn nine () -> Self {
                    9.
                }

                fn twenty_seven () -> Self {
                    27.
                }
            }
        )*
    };
}

pub trait Consts {
    fn two () -> Self;
    fn three () -> Self;
    fn four () -> Self;
    fn five () -> Self;
    fn six () -> Self;
    fn seven () -> Self;
    fn eight () -> Self;
    fn nine () -> Self;
    fn twenty_seven () -> Self;
}

int_consts!(u8, u16, u32, u64, u128, usize);
int_consts!(i8, i16, i32, i64, i128, isize);
float_consts!(f32, f64);