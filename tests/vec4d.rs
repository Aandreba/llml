use llml::vec::{EucVecd4, EucVecf4};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecd4::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y(), alpha.z() $sy beta.z(), alpha.w() $sy beta.w()]))
    }
}

#[cfg(feature = "llml_serde")]
#[test]
fn serde () {
    let alpha : EucVecd4 = random();
    let json = serde_json::to_string(&alpha).unwrap();
    let beta : EucVecd4 = serde_json::from_str(json.as_str()).unwrap();

    assert!((alpha - beta).abs().sum() <= f64::EPSILON * 4.);
}

#[test]
fn eq () {
    assert_eq!(EucVecd4::new([1., 2., 3., 4.]), EucVecd4::new([1., 2., 3., 4.]));
    assert_ne!(EucVecd4::new([1., 2., 3., 4.]), EucVecd4::new([5., 6., 7., 8.]))
}

#[test]
fn into () {
    let alpha = EucVecd4::new([1., 2., 3., 4.]);
    assert_eq!(Into::<EucVecf4>::into(alpha), EucVecf4::new([1., 2., 3., 4.]));
    assert_eq!(Into::<[f64;4]>::into(alpha), [1., 2., 3., 4.])
}

#[test]
fn from_scalar () {
    let alpha : f64 = random();
    assert_eq!(EucVecd4::from_scal(alpha), EucVecd4::new([alpha, alpha, alpha, alpha]))
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

#[test]
fn abs () {
    let alpha : EucVecd4 = random();
    assert_eq!(alpha.abs(), EucVecd4::new([alpha.x().abs(), alpha.y().abs(), alpha.z().abs(), alpha.w().abs()]))
}

#[test]
fn sqrt () {
    let alpha : EucVecd4 = random();
    assert_eq!(alpha.sqrt(), EucVecd4::new([alpha.x().sqrt(), alpha.y().sqrt(), alpha.z().sqrt(), alpha.w().sqrt()]))
}

const RSQRT_EPSILON : f64 = 0.0003662109375 + f64::EPSILON;

#[test]
fn sqrt_fast () {
    let alpha : EucVecd4 = random();
    let fast = alpha.sqrt_fast();

    assert!((fast.x() - alpha.x().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.y() - alpha.y().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.z() - alpha.z().sqrt()).abs() <= RSQRT_EPSILON);
    assert!((fast.w() - alpha.w().sqrt()).abs() <= RSQRT_EPSILON);
}

fn get_vecs () -> (EucVecd4, EucVecd4) {
    (random(), random())
}