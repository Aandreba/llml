use llml::{EucVecd3};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecd3::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y(), alpha.z() $sy beta.z()]))
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
    let alpha : EucVecd3 = random();
    assert_eq!(-alpha, EucVecd3::new([-alpha.x(), -alpha.y(), -alpha.z()]))
}

#[test]
fn sum () {
    let alpha = EucVecd3::new([1., 2., 3.]);
    assert_eq!(alpha.sum(), alpha.x() + alpha.y() + alpha.z());
}


#[test]
fn dot () {
    let alpha = EucVecd3::new([1., 2., 3.]);
    let beta = EucVecd3::new([4., 5., 6.]);

    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y() + alpha.z() * beta.z())
}

#[test]
fn norm () {
    let alpha : EucVecd3 = random();
    assert_eq!(alpha.norm(), (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z()).sqrt())
}

#[test]
fn unit () {
    let alpha : EucVecd3 = random();

    let norm = (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z()).sqrt();
    assert_eq!(alpha.unit(), EucVecd3::new([alpha.x() / norm, alpha.y() / norm, alpha.z() / norm]))
}

#[test]
fn sqrt () {
    let alpha : EucVecd3 = random();
    assert_eq!(alpha.sqrt(), EucVecd3::new([alpha.x().sqrt(), alpha.y().sqrt(), alpha.z().sqrt()]))
}

const RSQRT_EPSILON : f64 = 0.0003662109375 + f64::EPSILON;

#[test]
fn sqrt_fast () {
    let alpha : EucVecd3 = random();
    let fast = alpha.sqrt_fast();

    assert!((fast.x() - alpha.x().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.y() - alpha.y().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.z() - alpha.z().sqrt()).abs() <= RSQRT_EPSILON);
}

fn get_vecs () -> (EucVecd3, EucVecd3) {
    (random(), random())
}