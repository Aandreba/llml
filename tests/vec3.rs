use llml::vec::{EucVecf3, EucVecd3};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecf3::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y(), alpha.z() $sy beta.z()]))
    }
}

#[test]
fn eq () {
    assert_eq!(EucVecf3::new([1., 2., 3.]), EucVecf3::new([1., 2., 3.]));
    assert_ne!(EucVecf3::new([1., 2., 3.]), EucVecf3::new([4., 5., 6.]))
}

#[test]
fn into () {
    let alpha = EucVecf3::new([1., 2., 3.]);
    assert_eq!(Into::<EucVecd3>::into(alpha), EucVecd3::new([1., 2., 3.]));
    assert_eq!(Into::<[f32;3]>::into(alpha), [1., 2., 3.])
}

#[test]
fn from_scalar () {
    let alpha : f32 = random();
    assert_eq!(EucVecf3::from_scalar(alpha), EucVecf3::new([alpha, alpha, alpha]))
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
    let alpha : EucVecf3 = random();
    assert_eq!(-alpha, EucVecf3::new([-alpha.x(), -alpha.y(), -alpha.z()]))
}

#[test]
fn sum () {
    let alpha = EucVecf3::new([1., 2., 3.]);
    assert_eq!(alpha.sum(), alpha.x() + alpha.y() + alpha.z());
}


#[test]
fn dot () {
    let alpha = EucVecf3::new([1., 2., 3.]);
    let beta = EucVecf3::new([4., 5., 6.]);

    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y() + alpha.z() * beta.z())
}

#[test]
fn norm () {
    let alpha : EucVecf3 = random();
    assert_eq!(alpha.norm(), (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z()).sqrt())
}

#[test]
fn unit () {
    let alpha : EucVecf3 = random();

    let norm = (alpha.x() * alpha.x() + alpha.y() * alpha.y() + alpha.z() * alpha.z()).sqrt();
    assert_eq!(alpha.unit(), EucVecf3::new([alpha.x() / norm, alpha.y() / norm, alpha.z() / norm]))
}

#[test]
fn abs () {
    let alpha : EucVecf3 = random();
    assert_eq!(alpha.abs(), EucVecf3::new([alpha.x().abs(), alpha.y().abs(), alpha.z().abs()]))
}

#[test]
fn sqrt () {
    let alpha : EucVecf3 = random();
    assert_eq!(alpha.sqrt(), EucVecf3::new([alpha.x().sqrt(), alpha.y().sqrt(), alpha.z().sqrt()]))
}

#[test]
fn sqrt_fast () {
    // MANUALLY ASSERTED
}

fn get_vecs () -> (EucVecf3, EucVecf3) {
    (random(), random())
}