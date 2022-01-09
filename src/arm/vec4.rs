arm_use!();
use core::mem::transmute;
use std::ptr::{addr_of, addr_of_mut};
use std::{ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}};
use crate::EucVecd2;

macro_rules! impl_vec4_vv {
    ($target:ident, $ty:ident, $tag:ident) => {
        impl_vec4_vv!(
            $target, $ty, $tag,
            Add, add, +,
            Sub, sub, -,
            Mul, mul, *,
            Div, div, /
        );

        impl $target {
            #[inline]
            pub fn new (x: $ty, y: $ty, z: $ty, w: $ty) -> Self {
                unsafe { Self(transmute([x, y]), transmute([z, w])) }
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                self.0.x()
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                self.0.y()
            }

            #[inline(always)]
            pub fn z (&self) -> $ty {
                self.1.x()
            }

            #[inline(always)]
            pub fn w (&self) -> $ty {
                self.1.y()
            }

            #[inline(always)]
            pub fn sum (self) -> $ty {
                self.0.sum() + self.1.sum()
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }
        }

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }

        impl Index<usize> for $target {
            type Output = $ty;

            #[inline(always)]
            fn index (&self, idx: usize) -> &$ty {
                unsafe {
                    match idx {
                        0 => &*(addr_of!(self.0) as *const $ty),
                        1 => &*(addr_of!(self.0) as *const $ty).add(1),
                        2 => &*(addr_of!(self.1) as *const $ty),
                        3 => &*(addr_of!(self.1) as *const $ty).add(1),
                        _ => panic!("Index '{}' out of bounds", idx)
                    }
                }
            }
        }

        impl IndexMut<usize> for $target {
            #[inline(always)]
            fn index_mut (&mut self, idx: usize) -> &mut $ty {
                unsafe {
                    match idx {
                        0 => &mut *(addr_of_mut!(self.0) as *mut $ty),
                        1 => &mut *(addr_of_mut!(self.0) as *mut $ty).add(1),
                        2 => &mut *(addr_of_mut!(self.1) as *mut $ty),
                        3 => &mut *(addr_of_mut!(self.1) as *mut $ty).add(1),
                        _ => panic!("Index '{}' out of bounds", idx)
                    }
                }
            }
        }

        impl PartialEq for $target {
            #[inline(always)]
            fn eq (&self, rhs: &Self) -> bool {
                self.1 == rhs.1 && self.0 == rhs.0
            }
        }

        impl Into<[$ty;4]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;4] {
                unsafe { transmute(self) }
            }
        }
    };

    ($target:ident, $ty:ident, $tag:ident, $($trait:ident, $fun:ident, $sy:tt),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0 $sy rhs.0, self.1 $sy rhs.1)
                }
            }

            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0 $sy rhs, self.1 $sy rhs)
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self $sy rhs.0, self $sy rhs.1)
                }
            }
        )*
    };
}

wrap!(EucVecf4, float32x4_t);
impl_vec4!(EucVecf4, f32, q, u32);

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct EucVecd4 (pub(crate) EucVecd2, pub(crate) EucVecd2);
impl Eq for EucVecd4 {}

impl_vec4_vv!(EucVecd4, f64, q);