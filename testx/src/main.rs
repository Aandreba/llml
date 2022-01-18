use llml::{Matf2, EucVecf2};

fn main () {
    unsafe {
        let alpha = Matf2::new([1., 2., 3., 4.]);
        let beta = EucVecf2::new([9., 8.]);
        print!("{:?}", alpha * beta);
    }
}