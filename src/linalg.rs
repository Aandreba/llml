use std::alloc::alloc_zeroed;

use num::Num;
use crate::{mat::seq::Mat, array::{build_array, allocate_array, malloc}};

pub fn bareiss<T: Num + Copy + Default, const R: usize, const C: usize> (matrix: Mat<T,R,C>) -> Option<Mat<T,R,C>> {
    if matrix[0].into_iter().any(|x| x.is_zero()) {
        return None
    }

    let mut result = Fract
    unsafe { result = malloc(); }

    let m00 = T::one();
    
    for k in 0..R-1 {
        let kp1 = k + 1;

        for i in kp1..R {
            for j in kp1..C {
                let num = matrix[i][j] * matrix[k][k] - matrix[];
            }
        }
    }

    Some(result)
}