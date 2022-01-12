use llml::{EucVecf4, EucVecd4};

fn main () {
    let alpha= EucVecf4::new(1., 2., 3., 4.);
    let beta : EucVecd4 = alpha.into();

    println!("{:?} {:?}", alpha, beta);
}