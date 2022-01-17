use llml::{EucVecd4};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecd4::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y(), alpha.z() $sy beta.z(), alpha.w() $sy beta.w()]))
    }
}

#[test]
fn add () {
    test_arith!(+);
}

#[test]
fn sub () {
    test_arith!(-);
}

#[test]
fn mul () {
    test_arith!(*);
}

#[test]
fn div () {
    test_arith!(/);
}

#[test]
fn neg () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);
    assert_eq!(-alpha, EucVecd4::new([-alpha.x(), -alpha.y(), -alpha.z(), -alpha.w()]))
}

#[test]
fn sum () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);
    assert_eq!(alpha.sum(), alpha.x() + alpha.y() + alpha.z() + alpha.w())
}


#[test]
fn dot () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);
    let beta = EucVecd4::new([5., 6., 7., 8.]);
    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y() + alpha.z() * beta.z() + alpha.w() * beta.w())
}

#[test]
fn norm () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);
    assert_eq!(alpha.norm(), (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z() + alpha.w() * alpha.w()).sqrt())
}

#[test]
fn unit () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);

    let norm = (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z() + alpha.w() * alpha.w()).sqrt();
    assert_eq!(alpha.unit(), EucVecd4::new([alpha.x() / norm, alpha.y() / norm, alpha.z() / norm, alpha.w() / norm]))
}

fn get_vecs () -> (EucVecd4, EucVecd4) {
    (random(), random())
}