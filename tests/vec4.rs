use llml::{EucVecf4, traits::Sqrt};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecf4::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y(), alpha.z() $sy beta.z(), alpha.w() $sy beta.w()]))
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
    let alpha = EucVecf4::new([1., 2., 3., 4.]);
    assert_eq!(-alpha, EucVecf4::new([-alpha.x(), -alpha.y(), -alpha.z(), -alpha.w()]))
}

#[test]
fn sum () {
    let alpha = EucVecf4::new([1., 2., 3., 4.]);
    assert_eq!(alpha.sum(), alpha.x() + alpha.y() + alpha.z() + alpha.w())
}


#[test]
fn dot () {
    let alpha = EucVecf4::new([1., 2., 3., 4.]);
    let beta = EucVecf4::new([5., 6., 7., 8.]);
    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y() + alpha.z() * beta.z() + alpha.w() * beta.w())
}

#[test]
fn norm () {
    let alpha = EucVecf4::new([1., 2., 3., 4.]);
    assert_eq!(alpha.norm(), (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z() + alpha.w() * alpha.w()).sqrt())
}

#[test]
fn unit () {
    let alpha = EucVecf4::new([1., 2., 3., 4.]);

    let norm = (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z() + alpha.w() * alpha.w()).sqrt();
    assert_eq!(alpha.unit(), EucVecf4::new([alpha.x() / norm, alpha.y() / norm, alpha.z() / norm, alpha.w() / norm]))
}

#[test]
fn sqrt () {
    let alpha : EucVecf4 = random();
    assert_eq!(alpha.sqrt(), EucVecf4::new([alpha.x().sqrt(), alpha.y().sqrt(), alpha.z().sqrt(), alpha.w().sqrt()]))
}

const RSQRT_EPSILON : f32 = 0.0003662109375 + f32::EPSILON;

#[test]
fn sqrt_fast () {
    let alpha : EucVecf4 = random();
    let fast = alpha.sqrt_fast();

    assert!((fast.x() - alpha.x().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.y() - alpha.y().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.z() - alpha.z().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.w() - alpha.w().sqrt()).abs() <= RSQRT_EPSILON);
}

fn get_vecs () -> (EucVecf4, EucVecf4) {
    (random(), random())
}