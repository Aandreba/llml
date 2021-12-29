use std::{ops::{Add, Sub, Mul, Div}};
use crate::{EucVecf3, EucVecf2};

type CastTarget = (EucVecf2, f32);
macro_rules! map_casted {
    ($lhs:ident, $trait:ident, $fn:ident, $f:expr) => {
        map_to_trait!($lhs, $trait, $fn, |x: Self, y: Self| {
            $f(x.casted(), y.casted())
        });
    };
}

// Vector - Vector
map_casted!(EucVecf3, Add, add, |x: CastTarget, y: CastTarget| Self::unsafe_from(x.0 + y.0, x.1 + y.1));
map_casted!(EucVecf3, Sub, sub, |x: CastTarget, y: CastTarget| Self::unsafe_from(x.0 - y.0, x.1 - y.1));
map_casted!(EucVecf3, Mul, mul, |x: CastTarget, y: CastTarget| Self::unsafe_from(x.0 * y.0, x.1 * y.1));
map_casted!(EucVecf3, Div, div, |x: CastTarget, y: CastTarget| Self::unsafe_from(x.0 / y.0, x.1 / y.1));

// Vector - Scalar Addition
map_to_trait!(EucVecf3, Add, f32, add, EucVecf3, |x: Self, y: f32| {
    let x = x.casted();
    Self::unsafe_from(x.0 + y, x.1 + y)
});

map_to_trait!(f32, Add, EucVecf3, add, EucVecf3, |x: Self, y: EucVecf3| {
    let y = y.casted();
    EucVecf3::unsafe_from(x + y.0, x + y.1)
});

// Vector - Scalar Subtraction
map_to_trait!(EucVecf3, Sub, f32, sub, EucVecf3, |x: Self, y: f32| {
    let x = x.casted();
    Self::unsafe_from(x.0 - y, x.1 - y)
});

map_to_trait!(f32, Sub, EucVecf3, sub, EucVecf3, |x: Self, y: EucVecf3| {
    let y = y.casted();
    EucVecf3::unsafe_from(x - y.0, x - y.1)
});

// Vector - Scalar Multiplication
map_to_trait!(EucVecf3, Mul, f32, mul, EucVecf3, |x: Self, y: f32| {
    let x = x.casted();
    Self::unsafe_from(x.0 * y, x.1 * y)
});

map_to_trait!(f32, Mul, EucVecf3, mul, EucVecf3, |x: Self, y: EucVecf3| {
    let y = y.casted();
    EucVecf3::unsafe_from(x * y.0, x * y.1)
});

// Vector - Scalar Division
map_to_trait!(EucVecf3, Div, f32, div, EucVecf3, |x: Self, y: f32| {
    let x = x.casted();
    Self::unsafe_from(x.0 / y, x.1 / y)
});

map_to_trait!(f32, Div, EucVecf3, div, EucVecf3, |x: Self, y: EucVecf3| {
    let y = y.casted();
    EucVecf3::unsafe_from(x / y.0, x / y.1)
});

impl EucVecf3 {
    #[inline(always)]
    pub(crate) unsafe fn casted (self) -> (EucVecf2, f32) {
        let ptr = &self as *const EucVecf3;
        (*(ptr as *const EucVecf2), *(ptr as *const f32).offset(2))
    }

    #[inline(always)]
    pub(crate) unsafe fn unsafe_from_tuple (x: (EucVecf2, f32)) -> Self {
        let ptr = &x as *const (EucVecf2, f32) as *const f32;
        Self::new(*ptr, *ptr.offset(1), *ptr.offset(2))
    }

    #[inline(always)]
    pub(crate) unsafe fn unsafe_from (x: EucVecf2, y: f32) -> Self {
        Self::unsafe_from_tuple((x, y))
    }

    #[inline(always)]
    pub fn sum (self) -> f32 {
        unsafe {
            (*(&self as *const EucVecf3 as *const EucVecf2)).sum() + self.z
        }
    }
    
    #[inline(always)]
    pub fn dot (self, rhs: Self) -> f32 {
        unsafe {
            let alpha = *(&self as *const EucVecf3 as *const EucVecf2);
            let beta = *(&self as *const EucVecf3 as *const EucVecf2);
            self.z.mul_add(rhs.z, alpha.dot(beta))
        }
    }

    #[inline(always)]
    pub fn cross (self, rhs: Self) -> Self {
        todo!()
    }
}