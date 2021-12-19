use std::{ops::{Add, Index, IndexMut, Sub, Mul, Div, Neg}, fmt::{Debug}, future};
use num::{Num, traits::real::Real, Signed, Complex};
use rayon::{iter::{IntoParallelIterator, IndexedParallelIterator, ParallelIterator}};
use crate::{extra::array::{build_array, build_array_mt}, arith, scal_arith, frac::Fraction};

use super::seq::EucVec;

/* -- TYPES -- */
// LENGTH TYPES
pub type EucVecMt2<T> = EucVecMt<T,2>;
pub type EucVecMt3<T> = EucVecMt<T,3>;
pub type EucVecMt4<T> = EucVecMt<T,4>;

// PARAM TYPES
pub type EucVecMtf<const N: usize> = EucVecMt<f32,2>;
pub type EucVecMtd<const N: usize> = EucVecMt<f64,3>;

// FULL TYPES
pub type EucVecMtf2 = EucVecMt<f32,2>;
pub type EucVecMtf3 = EucVecMt<f32,3>;
pub type EucVecMtf4 = EucVecMt<f32,4>;

pub type EucVecMtd2 = EucVecMt<f64,2>;
pub type EucVecMtd3 = EucVecMt<f64,3>;
pub type EucVecMtd4 = EucVecMt<f64,4>;

/* -- DEFINITION -- */
#[derive(Debug, Copy, Clone)]
pub struct EucVecMt<T: Num, const N: usize> ([T;N]);

impl<T: Num + Copy + Send + Sync, const N: usize> EucVecMt<T,N> {
    pub fn new (x: [T;N]) -> Self {
        EucVecMt(x)
    }

    pub fn as_seq (self) -> EucVec<T,N> {
        EucVec::new(self.0)
    }

    pub fn into_iter (self) -> rayon::array::IntoIter<T,N> {
        self.0.into_par_iter()
    }

    pub fn dot (self, other: EucVecMt<T,N>) -> T {
        self.0.into_par_iter()
            .zip(other.0)
            .map(|(x, y)| x * y)
            .reduce(|| T::zero(), |x, y| x + y)
    }

    pub fn norm2 (self) -> T {
        self.0.into_par_iter()
            .map(|x| x * x)
            .reduce(|| T::zero(), |x, y| x + y)
    }

    pub fn norm (self) -> T where T: Real {
        self.norm2().sqrt()
    }

    pub fn unit (self) -> EucVecMt<T,N> where T: Real + Debug {
        let norm = self.norm();
        println!("{:?} {:?}", self,norm);
        self / norm
    }

    pub fn sum (self) -> Option<T> {
        match N {
            0|1 => None,
            _ => Some(self.0.into_par_iter().reduce(|| T::zero(), |x, y| x + y))
        }
    }

    pub fn abs (self) -> EucVecMt<T,N> where T: Signed {
        Self(build_array_mt(|i| self[i].abs()))
    }
}

// VECTOR - VECTOR
arith!(Add, EucVecMt<T, N>, add, |x : &EucVecMt<T,N>, y : &EucVecMt<T,N>| {
    let array : [T; N] = build_array_mt(|i| x.0[i] + y.0[i]);
    EucVecMt(array)
}, Send, Sync);

arith!(Sub, EucVecMt<T,N>, sub, |x : &EucVecMt<T,N>, y : &EucVecMt<T,N>| {
    let array : [T; N] = build_array_mt(|i| x.0[i] - y.0[i]);
    EucVecMt(array)
}, Send, Sync);

arith!(Mul, EucVecMt<T,N>, mul, |x : &EucVecMt<T,N>, y : &EucVecMt<T,N>| {
    let array : [T; N] = build_array_mt(|i| x.0[i] * y.0[i]);
    EucVecMt(array)
}, Send, Sync);

arith!(Div, EucVecMt<T,N>, div, |x : &EucVecMt<T,N>, y : &EucVecMt<T,N>| {
    let array : [T; N] = build_array_mt(|i| x.0[i] / y.0[i]);
    EucVecMt(array)
}, Send, Sync);

// SCALAR - VECTOR
scal_arith!(Add, EucVecMt<T,N>, add, |x : &EucVecMt<T,N>, y : &T| {
    let array : [T; N] = build_array_mt(|i| x.0[i] + y.clone());
    EucVecMt::<T,N>(array)
}, Send, Sync);

scal_arith!(Sub, EucVecMt<T,N>, sub, |x : &EucVecMt<T,N>, y : &T| {
    let array : [T; N] = build_array_mt(|i| x.0[i] - y.clone());
    EucVecMt::<T,N>(array)
}, Send, Sync);

scal_arith!(Mul, EucVecMt<T,N>, mul, |x : &EucVecMt<T,N>, y : &T| {
    let array : [T; N] = build_array_mt(|i| x.0[i] * y.clone());
    EucVecMt::<T,N>(array)
}, Send, Sync);

scal_arith!(Div, EucVecMt<T,N>, div, |x : &EucVecMt<T,N>, y : &T| {
    let array : [T;N] = build_array_mt(|i| T::one());
    EucVecMt::<T,N>(array)
}, Send, Sync);

// OTHER TRAITS
impl<O: Num + Copy + Send + Sync, T: Num + Copy + Send + Sync + Neg<Output = O>, const N: usize> Neg for EucVecMt<T,N> {
    type Output = EucVecMt<O,N>;

    fn neg (self) -> Self::Output {
        let array = build_array_mt(|i| -self[i]);
        EucVecMt(array)
    }
}

impl<T: Num + Copy + Send + Sync> EucVecMt3<T> {
    pub fn cross (self, other: EucVecMt3<T>) -> EucVecMt3<T> {
        EucVecMt3::new([
            self[1] * other[2] - self[2] * other[1],
            self[0] * other[2] - self[2] * other[0],
            self[0] * other[1] - self[1] * other[0]
        ])
    }
}

impl<T: Num, const N: usize> Index<usize> for EucVecMt<T,N>  {
    type Output = T;

    fn index (&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num, const N: usize> IndexMut<usize> for EucVecMt<T,N>  {
    fn index_mut (&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Num, const N: usize> Index<char> for EucVecMt<T,N>  {
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

impl<T: Num, const N: usize> IndexMut<char> for EucVecMt<T,N>  {
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

impl<T: Num + Default + Copy, const N: usize> Default for EucVecMt<T,N>  {
    fn default() -> Self {
        EucVecMt([T::default();N])
    }
}

// INTO's
impl<T: Num, const N: usize> Into<[T;N]> for EucVecMt<T,N> {
    fn into(self) -> [T;N] {
        self.0
    }
}

impl<T: Num + Copy + Send + Sync, const N: usize> Into<EucVecMt<Complex<T>,N>> for EucVecMt<T,N> {
    fn into(self) -> EucVecMt<Complex<T>,N> {
        let array = build_array_mt(|i| Complex::new(self[i], T::zero()));
        EucVecMt(array)
    }
}

impl<T: Num + Copy + Send + Sync, const N: usize> Into<EucVecMt<Fraction<T>,N>> for EucVecMt<T,N> {
    fn into(self) -> EucVecMt<Fraction<T>,N> {
        let array = build_array_mt(|i| Fraction::of_value(self[i]));
        EucVecMt(array)
    }
}

impl<T: Num + Copy + Send + Sync, const N: usize> Into<EucVecMt<T,N>> for EucVecMt<Complex<T>,N> {
    fn into(self) -> EucVecMt<T,N> {
        let array = build_array_mt(|i| self[i].re);
        EucVecMt(array)
    }
}