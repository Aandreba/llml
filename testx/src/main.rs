use llml::{EucVecf4, EucVecd4, EucVecd3, EucVecf3};

fn main () {
    let alpha= EucVecd4::new(1., 2., 3., 4.);
    let beta : EucVecf4 = alpha.into();

    println!("{:?} {:?}", alpha, beta);
}