use std::{ops::{Add, Sub, Mul, Index, Div, IndexMut, Neg}, rc::Rc, fmt::Debug, alloc::{Layout, alloc}};
use num::{Num, Float, ToPrimitive, FromPrimitive};
use crate::{vec::seq::EucVec, mat_arith, array::{build_array, collect_into_array_unchecked}, mat_scal_arith, extra::iter::TrustedSkip};

pub type SqrMat<T, const N: usize> = Mat<T, N, N>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mat<T: Num, const R: usize, const C: usize> ([EucVec<T,C>;R]);

impl<T: Num + Copy, const R: usize, const C: usize> Mat<T,R,C> {
    pub fn new (values: [EucVec<T,C>;R]) -> Self {
        Mat(values)
    }

    pub fn of_array (values: [[T;C];R]) -> Self where T: Copy {
        Mat(values.map(|x| EucVec::new(x)))
    }

    pub fn into_iter (self) -> std::array::IntoIter<EucVec<T,C>,R> {
        self.0.into_iter()
    } 

    pub fn rows (&self) -> usize {
        R
    }

    pub fn cols (&self) -> usize {
        C
    }

    pub fn rref (&self) -> Self {
        let mut rc = self.clone();

        for i in 0..R {
            if rc[i][i] != T::zero() {
                rc[i] = rc[i] / rc[i][i];
            }

            for j in 0..R {
                if i == j { continue; }
                rc[j] = rc[j] - (rc[i] * rc[j][i]); 
            }
        }

        rc
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
    pub fn identity () -> Self {
        let array = build_array(|i | {
            let vec = build_array(|j| if i == j { T::one() } else { T::zero() });
            EucVec::new(vec)
        });

        Self(array)
    }

    pub fn scal_mul (self, rhs: SqrMat<T,N>) -> Self {
        let array = build_array(|i| self[i] * rhs[i]);
        Self(array)
    }

    pub fn tr (&self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum = sum + self[i][i];
        }

        sum
    }

    pub fn inv (self) -> Option<Self> where T: Float {
        let mut rc = self.clone();
        let mut ident = Self::identity();
        let ident_clone = ident.clone();

        for i in 0..N {
            if rc[i][i] != T::zero() {
                let ii = rc[i][i];
                rc[i] = rc[i] / ii;
                ident[i] = ident[i] / ii;
            }

            for j in 0..N {
                if i == j { continue; }
                let ji = rc[j][i];
                rc[j] = rc[j] - (rc[i] * ji); 
                ident[j] = ident[j] - (ident[i] * ji);
            }
        }

        if rc == ident_clone {
            return Some(ident)
        }

        None
    }

    pub fn poly (self) -> [T;N+1] where T: Float + FromPrimitive  {
        let layout = Layout::new::<[T;N+1]>();
        let mut result;
        unsafe {
            result = *(alloc(layout) as *mut [T;N+1]);
        }

        result[0] = T::one();
        let mut clone = self.clone();

        for i in 1..=N {
            result[i] = -clone.tr() / FromPrimitive::from_usize(i).unwrap();
            clone = (self * clone) + result[i]
        }

        result
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

impl<T: Num + Copy, const R: usize, const C: usize> IndexMut<usize> for Mat<T,R,C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}