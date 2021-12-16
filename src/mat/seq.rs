use std::{ops::{Add, Sub, Mul, Index, Div}, rc::Rc, iter::TrustedLen};

use num::Num;
use crate::{vec::seq::EucVec, mat_arith, array::{build_array, collect_into_array_unchecked}, mat_scal_arith, extra::iter::{IterSlice, IterSkip}};

pub type SqrMat<T, const N: usize> = Mat<T, N, N>;

#[derive(Debug, Clone, Copy)]
pub struct Mat<T: Num, const R: usize, const C: usize> ([EucVec<T,C>;R]);

impl<T: Num, const R: usize, const C: usize> Mat<T,R,C> {
    pub fn new (values: [EucVec<T,C>;R]) -> Self {
        Mat(values)
    }

    pub fn of_array (values: [[T;C];R]) -> Self where T: Copy {
        Mat(values.map(|x| EucVec::new(x)))
    }

    pub fn rows () -> usize {
        R
    }

    pub fn cols () -> usize {
        C
    }
}

mat_arith!(Add, Mat<T,R,C>, add, |x : &Mat<T,R,C>, y : &Mat<T,R,C>| {
    let array = build_array(|i| x.0[i] + y.0[i]);
    Mat(array)
});

mat_arith!(Sub, Mat<T,R,C>, sub, |x : &Mat<T,R,C>, y : &Mat<T,R,C>| {
    let array = build_array(|i| x.0[i] - y.0[i]);
    Mat(array)
});

// MATRIX - MATRIX MULTIPLICATION
impl<T: Num + Copy, const R: usize, const C: usize, const N: usize> Mul<Mat<T,C,N>> for Mat<T,R,C> {
    type Output = Mat<T, R, N>;

    fn mul(self, rhs: Mat<T,C,N>) -> Self::Output {
        let array = build_array(|i| {
            let sub = build_array(|j| {
                let mut sum = T::zero();
                for k in 0..C {
                    sum = sum + self[i][k] * rhs[k][j];
                }

                sum
            });

            EucVec::new(sub)
        });

        Mat(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize, const N: usize> Mul<&Mat<T,C,N>> for Mat<T,R,C> {
    type Output = Mat<T, R, N>;

    fn mul(self, rhs: &Mat<T,C,N>) -> Self::Output {
        let array = build_array(|i| {
            let sub = build_array(|j| {
                let mut sum = T::zero();
                for k in 0..C {
                    sum = sum + self[i][k] * rhs[k][j];
                }

                sum
            });

            EucVec::new(sub)
        });

        Mat(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize, const N: usize> Mul<Mat<T,C,N>> for &Mat<T,R,C> {
    type Output = Mat<T, R, N>;

    fn mul(self, rhs: Mat<T,C,N>) -> Self::Output {
        let array = build_array(|i| {
            let sub = build_array(|j| {
                let mut sum = T::zero();
                for k in 0..C {
                    sum = sum + self[i][k] * rhs[k][j];
                }

                sum
            });

            EucVec::new(sub)
        });

        Mat(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize, const N: usize> Mul<&Mat<T,C,N>> for &Mat<T,R,C> {
    type Output = Mat<T, R, N>;

    fn mul(self, rhs: &Mat<T,C,N>) -> Self::Output {
        let array = build_array(|i| {
            let sub = build_array(|j| {
                let mut sum = T::zero();
                for k in 0..C {
                    sum = sum + self[i][k] * rhs[k][j];
                }

                sum
            });

            EucVec::new(sub)
        });

        Mat(array)
    }
}

// SCALAR
mat_scal_arith!(Add, Mat<T, R, C>, add, |x : &Mat<T,R,C>, y: &T| {
    let array = build_array(|i| x.0[i] + y.clone());
    Mat(array)
});

mat_scal_arith!(Sub, Mat<T, R, C>, sub, |x : &Mat<T,R,C>, y: &T| {
    let array = build_array(|i| x.0[i] - y.clone());
    Mat(array)
});

mat_scal_arith!(Mul, Mat<T, R, C>, mul, |x : &Mat<T,R,C>, y: &T| {
    let array = build_array(|i| x.0[i] * y.clone());
    Mat(array)
});

mat_scal_arith!(Div, Mat<T, R, C>, div, |x : &Mat<T,R,C>, y: &T| {
    let array = build_array(|i| x.0[i] / y.clone());
    Mat(array)
});

// SQUARE MATRIX PROPERTIES
impl<T: Num + Copy, const N: usize> SqrMat<T, N> {
    pub fn tr (&self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum = sum + self[i][i];
        }

        sum
    }
}

// OTHER TRAITS
impl<T: Num + Copy, const R: usize, const C: usize> Index<usize> for Mat<T,R,C> {
    type Output = EucVec<T,C>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Index<usize> for &Mat<T,R,C> {
    type Output = EucVec<T,C>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}