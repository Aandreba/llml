use std::{ops::{Add, Sub, Mul, Index, Div, IndexMut, Neg}, fmt::Debug, alloc::{Layout, alloc}};
use derive_more::Neg;
use num::{Num, Float, FromPrimitive, Complex, Zero, One, Integer, traits::real::Real};
use crate::{vec::seq::EucVec, mat_arith, array::{build_array, allocate_array}, mat_scal_arith, extra::{consts::Consts, iter::IterJump}, poly::{self}, frac::Fraction};

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

    pub fn rref (self) -> Self {
        let mut rc : Mat<Fraction<T>,R,C> = self.into();

        for i in 0..R {
            if !rc[i][i].is_zero() {
                rc[i] = rc[i] / rc[i][i];
            }

            for j in 0..R {
                if i == j { continue; }
                rc[j] = rc[j] - (rc[i] * rc[j][i]); 
            }
        }

        rc.into()
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

// MATRIX - VECTOR MULTIPLICATION
impl<T: Num + Copy, const R: usize, const C: usize> Mul<EucVec<T,C>> for Mat<T,R,C> {
    type Output = EucVec<T,R>;

    fn mul(self, rhs: EucVec<T,C>) -> Self::Output {
        let array = build_array(|i| self[i].dot(rhs));
        EucVec::new(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Mul<&EucVec<T,C>> for Mat<T,R,C> {
    type Output = EucVec<T,R>;

    fn mul(self, rhs: &EucVec<T,C>) -> Self::Output {
        let array = build_array(|i| self[i].dot(rhs.clone()));
        EucVec::new(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Mul<EucVec<T,C>> for &Mat<T,R,C> {
    type Output = EucVec<T,R>;

    fn mul(self, rhs: EucVec<T,C>) -> Self::Output {
        let array = build_array(|i| self[i].dot(rhs));
        EucVec::new(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Mul<&EucVec<T,C>> for &Mat<T,R,C> {
    type Output = EucVec<T,R>;

    fn mul(self, rhs: &EucVec<T,C>) -> Self::Output {
        let array = build_array(|i| self[i].dot(rhs.clone()));
        EucVec::new(array)
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

    pub fn of_diag_scal (value: T) -> Self {
        let array = build_array(|i | {
            let vec = build_array(|j| if i == j { value } else { T::zero() });
            EucVec::new(vec)
        });

        Self(array)
    }

    pub fn identity () -> Self {
        Self::of_diag_scal(T::one())
    }

    pub fn scal_mul (self, rhs: SqrMat<T,N>) -> Self {
        let array = build_array(|i| self[i] * rhs[i]);
        Self(array)
    }

    // CONDITIONALS
    pub fn is_diag (&self) -> bool {
        (0..N).into_iter()
            .flat_map(|i| IterJump::new(self[i].into_iter(), i))
            .all(|x| x.is_zero())
    }

    /// SOLVE LINEAR EQUATIONS
    pub fn solve_inv (self, out: EucVec<T,N>) -> Option<EucVec<T,N>> {
        self.inv().map(|inv| inv * out)
    }

    pub fn solve_gauss (self, out: EucVec<T,N>) -> Option<EucVec<T,N>> {
        let mut rc : SqrMat<Fraction<T>,N> = self.into();
        let mut result : EucVec<Fraction<T>,N> = out.into();

        for i in 0..N {
            if !rc[i][i].is_zero() {
                let div = rc[i][i];
                rc[i] = rc[i] / div;
                result[i] = result[i] / div;
            }

            for j in 0..N {
                if i == j { continue; }

                let alpha = rc[j][i];
                rc[j] = rc[j] - (rc[i] * alpha);
                result[j] = result[j] - (result[i] * alpha); 
            }
        }

        if rc.is_diag() {
            return Some(result.into())
        }

        None
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
        let n = N-1;
        self.bareiss().map(|x| x[n][n])
    }
    
    pub fn inv (self) -> Option<Self> {
        let mut rc : SqrMat<Fraction<T>,N> = self.into();
        let mut ident = SqrMat::<Fraction<T>,N>::identity();
        let ident_clone = ident.clone();

        for i in 0..N {
            if !rc[i][i].is_zero() {
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
            return Some(ident.into())
        }

        None
    }

    pub fn bareiss (self) -> Option<Self> {
        if self[0].into_iter().any(|x| x.is_zero()) {
            return None
        }
    
        let mut result : SqrMat<Fraction<T>,N> = self.into();
        
        for k in 0..N-1 {
            let kp1 = k + 1;
            let div = if k == 0 { Fraction::one() } else { 
                let km1 = k - 1;
                result[km1][km1]
            };
    
            for i in kp1..N {
                for j in kp1..N {
                    let num = result[i][j] * result[k][k] - result[i][k] * result[k][i];
                    result[i][j] = num / div
                }
            }
        }
    
        Some(result.into())
    }

    pub fn poly (self) -> [T; N+1] where T: Float + FromPrimitive  {
        let mut result;
        unsafe { result = allocate_array() }

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
    T: Float + Consts + FromPrimitive + Debug, 
    [T;N+1]: Sized,
    [Complex<T>; {N+1}-1]: Sized {

        let complex : SqrMat<Complex<T>,N> = self.into();
        let vals = self.eigvals(limit);

        for i in 0..N {
            let submat = complex - SqrMat::of_diag_scal(vals[i]);
            println!("{:?}", submat)
        }

        todo!()

    }

    pub fn exp (self, limit: usize) -> Self where T: FromPrimitive {
        let mut result = Self::identity();
        let mut pow = result.clone();
        let mut div = T::one();

        for i in 1..=limit {
            pow = pow * self;
            div = div * T::from_usize(i).unwrap();
            
            let delta = pow / div;
            if delta.into_iter().all(|x| x.into_iter().all(|y| y.is_zero())) {
                break;
            }

            result = result + delta;
        }

        result
    }

    // DIAGONALIZABLE
    pub fn ln (self, limit: usize) -> Option<SqrMat<Complex<T>,N>> where 
    T: Float + Consts + FromPrimitive + Debug, 
    [T;N+1]: Sized,
    [Complex<T>; {N+1}-1]: Sized {
        let complx : SqrMat<Complex<T>,N> = self.into();
        let vectors = self.eigvecs(limit);
        let inv = vectors.inv();

        match inv {
            None => None,
            Some(inv) => {
                let alpha = inv * complx * vectors;
                let mut alpha_log = alpha.clone();
                for i in 0..N {
                    alpha_log[i][i] = alpha_log[i][i].ln();
                }

                Some(vectors * alpha_log * inv)
            }
        }
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

impl<T: Num + Copy, const R: usize, const C: usize> Into<Mat<Fraction<T>,R,C>> for Mat<T,R,C> {
    fn into(self) -> Mat<Fraction<T>,R,C> {
        Mat(self.0.map(|x| x.into()))
    }
}

impl<T: Num + Clone, const R: usize, const C: usize> Into<Mat<T,R,C>> for Mat<Complex<T>,R,C> {
    fn into(self) -> Mat<T,R,C> {
        Mat(self.0.map(|x| x.into()))
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Into<Mat<T,R,C>> for Mat<Fraction<T>,R,C> {
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