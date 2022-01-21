use llml::vec::{EucVecf2, EucVecd2};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecf2::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y()]))
    }
}

#[cfg(feature = "llml_serde")]
#[test]
fn serde () {
    let alpha : EucVecf2 = random();
    let json = serde_json::to_string(&alpha).unwrap();
    let beta : EucVecf2 = serde_json::from_str(json.as_str()).unwrap();

    assert_eq!(alpha, beta);
}

#[test]
fn eq () {
    assert_eq!(EucVecf2::new([1., 2.]), EucVecf2::new([1., 2.]));
    assert_ne!(EucVecf2::new([1., 2.]), EucVecf2::new([3., 3.]))
}

#[test]
fn into () {
    let alpha = EucVecf2::new([1., 2.]);
    assert_eq!(Into::<EucVecd2>::into(alpha), EucVecd2::new([1., 2.]));
    assert_eq!(Into::<[f32;2]>::into(alpha), [1., 2.])
}

#[test]
fn from_scalar () {
    let alpha : f32 = random();
    assert_eq!(EucVecf2::from_scalar(alpha), EucVecf2::new([alpha, alpha]))
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
    let alpha : EucVecf2 = random();
    assert_eq!(-alpha, EucVecf2::new([-alpha.x(), -alpha.y()]))
}

#[test]
fn sum () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.sum(), alpha.x() + alpha.y())
}


#[test]
fn dot () {
    let (alpha, beta) = get_vecs();
    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y())
}

#[test]
fn norm () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.norm(), alpha.x().hypot(alpha.y()))
}

#[test]
fn unit () {
    let alpha : EucVecf2 = random();

    let norm = alpha.x().hypot(alpha.y());
    assert_eq!(alpha.unit(), EucVecf2::new([alpha.x() / norm, alpha.y() / norm]))
}

#[test]
fn abs () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.abs(), EucVecf2::new([alpha.x().abs(), alpha.y().abs()]))
}

#[test]
fn sqrt () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.sqrt(), EucVecf2::new([alpha.x().sqrt(), alpha.y().sqrt()]))
}

const RSQRT_EPSILON : f32 = 0.0003662109375 + f32::EPSILON;

#[test]
fn sqrt_fast () {
    let alpha : EucVecf2 = random();
    let fast = alpha.sqrt_fast();

    assert!((fast.x() - alpha.x().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.y() - alpha.y().sqrt()).abs() <= RSQRT_EPSILON);
}

fn get_vecs () -> (EucVecf2, EucVecf2) {
    (random(), random())
}