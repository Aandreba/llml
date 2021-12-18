use std::{ops::{Add, Sub, Mul, Index, Div, IndexMut, Neg}, fmt::Debug, alloc::{Layout, alloc}};
use derive_more::Neg;
use num::{Num, Float, FromPrimitive, Complex, Zero, One};
use crate::{vec::seq::EucVec, mat_arith, array::{build_array, allocate_array}, mat_scal_arith, extra::{consts::Consts, iter::IterJump}, poly::{self}};

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
    pub fn of_diag (values: [T;N]) -> Self {
        let array = build_array(|i | {
            let vec = build_array(|j| if i == j { values[i] } else { T::zero() });
            EucVec::new(vec)
        });

        Self(array)
    }

    pub fn of_diag_of_scal (value: T) -> Self {
        let array = build_array(|i | {
            let vec = build_array(|j| if i == j { value } else { T::zero() });
            EucVec::new(vec)
        });

        Self(array)
    }

    pub fn identity () -> Self {
        Self::of_diag_of_scal(T::one())
    }

    pub fn scal_mul (self, rhs: SqrMat<T,N>) -> Self {
        let array = build_array(|i| self[i] * rhs[i]);
        Self(array)
    }

    pub fn diag (self) -> EucVec<T,N> {
        let array = build_array(|i| self[i][i]);
        EucVec::new(array)
    }

    pub fn tr (&self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum = sum + self[i][i];
        }

        sum
    }

    pub fn det (self) -> Option<T> {
        let rref = self.rref();
        let iter = DiagIter::new(rref);
        let prod = iter.reduce(|x, y| x * y);

        
        prod.map(|x| x)
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

    pub fn poly (self) -> [T; N+1] where T: Float + FromPrimitive  {
        let mut result;
        unsafe {
            result = allocate_array()
        }

        result[0] = T::one();
        let mut clone = self.clone();

        for i in 1..=N {
            result[i] = -clone.tr() / FromPrimitive::from_usize(i).unwrap();
            clone = (self * clone) + (self * result[i])
        }

        result
    }

    pub fn eigvals (self, limit: usize) -> EucVec<Complex<T>,N> where 
    T: Float + Consts + FromPrimitive, 
    [T;N+1]: Sized,
    [Complex<T>; {N+1}-1]: Sized {
        let fadlev : [T;N+1] = self.poly();
        let poly = poly::poly(limit, fadlev);

        unsafe { *(poly.as_ptr() as *const EucVec<Complex<T>,N>) }
    }

    pub fn eigvecs (self, limit: usize) -> SqrMat<Complex<T>,N> where 
    T: Float + Consts + FromPrimitive + Debug + Default, 
    [T;N+1]: Sized,
    [Complex<T>; {N+1}-1]: Sized  {

        let complex : SqrMat<Complex<T>,N> = self.into();
        let vals = self.eigvals(limit);

        let mut result = SqrMat::<Complex<T>,N>::default();

        for i in 0..N {
            let mat = complex - SqrMat::diag_of_scal(vals[i]);
            let into : SqrMat<T,N> = mat.into();
            let rref = mat.rref();

            println!("{:?}", into);

            for j in 0..N {
                let last = if j == 0 { Complex::one() } else { result[i][j-1] };
                let row = rref[j];

                let itself = row[j];
                let non_zero : Vec<(usize, Complex<T>)> = IterJump::new(0..N, j)
                    .map(|k| (k, row[k]))
                    .filter(|(i, x)| !x.is_zero())
                    .collect();
                
                if non_zero.len() == 0 {
                    result[i][j] = Complex::zero();
                    continue;
                }
            }

            //println!("{:?}\n", result[i]);
        }
        
        todo!()
    }
}

// OTHER TRAITS
impl<O: Num + Copy, T: Num + Copy + Neg<Output = O>, const R: usize, const C: usize> Neg for Mat<T,R,C> {
    type Output = Mat<O,R,C>;

    fn neg (self) -> Self::Output {
        Mat(self.0.map(|x| -x))
    }
}

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

impl<T: Num + Default + Copy, const R: usize, const C: usize> Default for Mat<T,R,C>  {
    fn default() -> Self {
        Self([EucVec::default(); R])
    }
}

// INTO's
impl<T: Num + Clone, const R: usize, const C: usize> Into<Mat<Complex<T>,R,C>> for Mat<T,R,C> {
    fn into(self) -> Mat<Complex<T>,R,C> {
        Mat(self.0.map(|x| x.into()))
    }
}

impl<T: Num + Clone, const R: usize, const C: usize> Into<Mat<T,R,C>> for Mat<Complex<T>,R,C> {
    fn into(self) -> Mat<T,R,C> {
        Mat(self.0.map(|x| x.into()))
    }
}


// DIAGONAL ITERATOR
struct DiagIter<T: Num, const N: usize> {
    parent: SqrMat<T,N>,
    idx: usize
}

impl<T: Num, const N: usize> DiagIter<T,N> {
    pub fn new (parent: SqrMat<T,N>) -> Self {
        Self { parent, idx: 0 }
    }
}

impl<T: Num + Copy, const N: usize> Iterator for DiagIter<T,N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= N {
            return None
        }

        let value = self.parent[self.idx][self.idx];
        self.idx += 1;
        Some(value)
    }
}