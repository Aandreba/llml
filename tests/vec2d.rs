use llml::vec::{EucVecd2, EucVecf2};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecd2::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y()]))
    }
}

#[test]
fn eq () {
    assert_eq!(EucVecd2::new([1., 2.]), EucVecd2::new([1., 2.]));
    assert_ne!(EucVecd2::new([1., 2.]), EucVecd2::new([3., 3.]))
}

#[test]
fn into () {
    assert_eq!(Into::<EucVecf2>::into(EucVecd2::new([1., 2.])), EucVecf2::new([1., 2.]))
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
    let alpha : EucVecd2 = random();
    assert_eq!(-alpha, EucVecd2::new([-alpha.x(), -alpha.y()]))
}

#[test]
fn sum () {
    let alpha : EucVecd2 = random();
    assert_eq!(alpha.sum(), alpha.x() + alpha.y())
}


#[test]
fn dot () {
    let (alpha, beta) = get_vecs();
    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y())
}

#[test]
fn norm () {
    let alpha : EucVecd2 = random();
    assert_eq!(alpha.norm(), alpha.x().hypot(alpha.y()))
}

#[test]
fn unit () {
    let alpha : EucVecd2 = random();

    let norm = alpha.x().hypot(alpha.y());
    assert_eq!(alpha.unit(), EucVecd2::new([alpha.x() / norm, alpha.y() / norm]))
}

#[test]
fn sqrt () {
    let alpha : EucVecd2 = random();
    assert_eq!(alpha.sqrt(), EucVecd2::new([alpha.x().sqrt(), alpha.y().sqrt()]))
}

const RSQRT_EPSILON : f64 = 0.0003662109375 + f64::EPSILON;

#[test]
fn sqrt_fast () {
    let alpha : EucVecd2 = random();
    let fast = alpha.sqrt_fast();

    assert!((fast.x() - alpha.x().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.y() - alpha.y().sqrt()).abs() <= RSQRT_EPSILON);
}

fn get_vecs () -> (EucVecd2, EucVecd2) {
    (random(), random())
}