use std::{ops::{Add, Index, IndexMut, Sub, Mul, Div}, fmt::{Debug}};
use num::{Num, traits::real::Real, Signed};

use crate::{extra::array::build_array, arith, scal_arith};
use super::mt::EucVecMt;

/* -- TYPES -- */
// LENGTH TYPES
pub type EucVec2<T> = EucVec<T,2>;
pub type EucVec3<T> = EucVec<T,3>;
pub type EucVec4<T> = EucVec<T,4>;

// PARAM TYPES
pub type EucVecf<const N: usize> = EucVec<f32,2>;
pub type EucVecd<const N: usize> = EucVec<f64,3>;

// FULL TYPES
pub type EucVecf2 = EucVec<f32,2>;
pub type EucVecf3 = EucVec<f32,3>;
pub type EucVecf4 = EucVec<f32,4>;

pub type EucVecd2 = EucVec<f64,2>;
pub type EucVecd3 = EucVec<f64,3>;
pub type EucVecd4 = EucVec<f64,4>;

/* -- DEFINITION -- */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EucVec<T: Num, const N: usize> ([T;N]);

impl<T: Num + Copy, const N: usize> EucVec<T,N> {
    pub fn new (x: [T;N]) -> Self {
        EucVec(x)
    }

    pub fn as_par (self) -> EucVecMt<T,N> where T: Send + Sync {
        EucVecMt::new(self.0)
    }

    pub fn into_iter (self) -> std::array::IntoIter<T,N> {
        self.0.into_iter()
    }

    pub fn dot (self, other: EucVec<T,N>) -> T {
        let red = self.0.into_iter()
            .zip(other.0.into_iter())
            .map(|(x, y)| x * y)
            .reduce(|x, y| x + y);

        match red {
            None => T::zero(),
            Some(x) => x
        }
    }

    pub fn norm2 (self) -> T {
        self.dot(self)
    }

    pub fn norm (self) -> T where T: Real {
        self.norm2().sqrt()
    }

    pub fn unit (self) -> EucVec<T,N> where T: Real {
        self / self.norm()
    }

    pub fn sum (self) -> Option<T> {
        self.into_iter().reduce(|x, y| x + y)
    }

    pub fn abs (self) -> EucVec<T,N> where T: Signed {
        Self(self.0.map(|x| x.abs()))
    }
}

// VECTOR - VECTOR
arith!(Add, EucVec<T,N>, add, |x : &EucVec<T,N>, y : &EucVec<T,N>| {
    let array : [T; N] = build_array(|i| x.0[i] + y.0[i]);
    EucVec(array)
});

arith!(Sub, EucVec<T,N>, sub, |x : &EucVec<T,N>, y : &EucVec<T,N>| {
    let array : [T; N] = build_array(|i| x.0[i] - y.0[i]);
    EucVec(array)
});

arith!(Mul, EucVec<T,N>, mul, |x : &EucVec<T,N>, y : &EucVec<T,N>| {
    let array : [T; N] = build_array(|i| x.0[i] * y.0[i]);
    EucVec(array)
});

arith!(Div, EucVec<T,N>, div, |x : &EucVec<T,N>, y : &EucVec<T,N>| {
    let array : [T; N] = build_array(|i| x.0[i] / y.0[i]);
    EucVec(array)
});

// SCALAR - VECTOR
scal_arith!(Add, EucVec<T,N>, add, |x : &EucVec<T,N>, y : &T| {
    let array : [T; N] = build_array(|i| x.0[i] + y.clone());
    EucVec(array)
});

scal_arith!(Sub, EucVec<T,N>, sub, |x : &EucVec<T,N>, y : &T| {
    let array : [T; N] = build_array(|i| x.0[i] - y.clone());
    EucVec(array)
});

scal_arith!(Mul, EucVec<T,N>, mul, |x : &EucVec<T,N>, y : &T| {
    let array : [T; N] = build_array(|i| x.0[i] * y.clone());
    EucVec(array)
});

scal_arith!(Div, EucVec<T,N>, div, |x : &EucVec<T,N>, y : &T| {
    let array : [T; N] = build_array(|i| x.0[i] / y.clone());
    EucVec(array)
});

// OTHER TRAITS
impl<T: Num + Copy> EucVec3<T> {
    pub fn cross (self, other: EucVec3<T>) -> EucVec3<T> {
        EucVec3::new([
            self[1] * other[2] - self[2] * other[1],
            self[0] * other[2] - self[2] * other[0],
            self[0] * other[1] - self[1] * other[0]
        ])
    }
}

impl<T: Num, const N: usize> Index<usize> for EucVec<T,N>  {
    type Output = T;

    fn index (&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num, const N: usize> IndexMut<usize> for EucVec<T,N>  {
    fn index_mut (&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Num, const N: usize> Index<char> for EucVec<T,N>  {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.0[0],
            'y' => &self.0[1],
            'z' => &self.0[2],
            'w' => &self.0[3],
            _ => panic!("Invalid index")
        }
    }
}

impl<T: Num, const N: usize> IndexMut<char> for EucVec<T,N>  {
    fn index_mut (&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.0[0],
            'y' => &mut self.0[1],
            'z' => &mut self.0[2],
            'w' => &mut self.0[3],
            _ => panic!("Invalid index")
        }
    }
}

impl<T: Num, const N: usize> Into<[T;N]> for EucVec<T,N> {
    fn into(self) -> [T;N] {
        self.0
    }
}

impl<T: Num + Default + Copy, const N: usize> Default for EucVec<T,N>  {
    fn default() -> Self {
        EucVec([T::default();N])
    }
}