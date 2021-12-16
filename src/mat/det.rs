use std::{marker::PhantomData, ops::Index, rc::Rc, fmt::Debug};

use num::Num;
use super::seq::SqrMat;

// MAT ITER
#[derive(Debug)]
enum PseudoParent<T: Num, const N: usize> {
    Mat (Rc<SqrMat<T,N>>),
    Slice (Rc<PseudoSlice<T,N>>)
}

impl<T: Num, const N: usize> PseudoParent<T,N> {
    fn len (&self) -> usize {
        match self {
            PseudoParent::Mat(_) => N,
            PseudoParent::Slice(x) => x.parent.len() - 1
        }
    }
}

#[derive(Debug)]
struct PseudoSlice<T: Num, const N: usize> {
    pub parent: PseudoParent<T,N>,
    idx: usize
}

impl<T: Num, const N: usize> PseudoSlice<T,N> {
    fn slice_mat (parent: &Rc<SqrMat<T,N>>, idx: usize) -> Self {
        PseudoSlice { parent: PseudoParent::Mat(parent.clone()), idx }
    }

    fn slice (self: &Rc<Self>, idx: usize) -> PseudoSlice<T,N> {
        PseudoSlice { parent: PseudoParent::Slice(self.clone()), idx }
    }
}

impl<T: Num + Copy, const N: usize> PseudoSlice<T,N> {
    fn len (&self) -> usize {
        self.parent.len() - 1
    }

    fn get (&self, row: usize, col: usize) -> T {
        let row = row + 1;
        let col = col + if col >= self.idx { 1 } else { 0 };

        match &self.parent {
            PseudoParent::Mat(x) => x[row][col],
            PseudoParent::Slice(x) => x.get(row, col)
        }
    }

    fn det (self) -> T {
        let len = self.len();
        match len {
            0 => T::zero(),
            1 => self.get(0, 0),
            2 => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => {
                let mut det = T::zero();
                let mut add = true;
                
                let rc = Rc::new(self);
                for i in 0..len {
                    let slice : PseudoSlice<T,N> = rc.slice(i);
                    let value = rc.get(0, i) * slice.det();

                    det = if add { det + value } else { det - value };
                    add = !add;
                }

                det
            }
        }
    }
}

impl<T: Num + Copy, const N: usize> SqrMat<T,N> {
    pub fn det (self) -> T {
        match N {
            0 => T::zero(),
            1 => self[0][0],
            2 => self[0][0] * self[1][1] * self[0][1] * self[1][0],
            _ => {
                let mut det = T::zero();
                let mut add = true;
                
                let rc = Rc::new(self);
                for i in 0..N {
                    let slice = PseudoSlice::slice_mat(&rc, i);
                    let value = self[0][i] * slice.det();

                    det = if add { det + value } else { det - value };
                    add = !add;
                }

                det
            }
        }
    }
}